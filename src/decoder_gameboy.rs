//! Nintendo Game Boy LR35902 instruction decoder.
//!
//! This deliberately does not reuse the Z80 opcode builders: LR35902 flag
//! meanings and its 16-bit address space differ in several important ways.

use super::cpu::Decoder;
use super::environment::Environment;
use super::opcode::Opcode;
use super::registers::{Reg16, Reg8};

const Z: u8 = 0x80;
const N: u8 = 0x40;
const H: u8 = 0x20;
const C: u8 = 0x10;
const R8: [Reg8; 8] = [
    Reg8::B,
    Reg8::C,
    Reg8::D,
    Reg8::E,
    Reg8::H,
    Reg8::L,
    Reg8::_HL,
    Reg8::A,
];
const RR: [Reg16; 4] = [Reg16::BC, Reg16::DE, Reg16::HL, Reg16::SP];

pub struct DecoderGameBoy {
    base: [Opcode; 256],
    cb: [Opcode; 256],
}

impl DecoderGameBoy {
    pub fn new() -> Self {
        Self {
            base: std::array::from_fn(|code| opcode(code as u8, false)),
            cb: std::array::from_fn(|code| opcode(code as u8, true)),
        }
    }
}

impl Decoder for DecoderGameBoy {
    fn decode(&self, env: &mut Environment) -> &Opcode {
        // LR35902 has a flat, wrapping 16-bit address space regardless of
        // any eZ80 state a caller may have configured before selecting this mode.
        env.state.reg.adl = false;
        env.state.reg.mbase = 0;
        env.state.set_pc(env.state.pc() as u16 as u32);
        let code = env.advance_pc();
        if code == 0xcb {
            &self.cb[env.advance_pc() as usize]
        } else {
            &self.base[code as usize]
        }
    }
}

fn opcode(code: u8, cb: bool) -> Opcode {
    let documented = if cb { true } else { documented_base(code) };
    Opcode {
        name: if cb {
            format!("CB ${:02X}", code)
        } else if documented {
            format!("GB ${:02X}", code)
        } else {
            "ILLEGAL".into()
        },
        action: Box::new(move |env| {
            if cb {
                execute_cb(env, code);
            } else if documented {
                execute_base(env, code);
            } else {
                env.trap_illegal_instruction();
            }
        }),
    }
}

fn documented_base(c: u8) -> bool {
    if (0x40..=0x7f).contains(&c) || (0x80..=0xbf).contains(&c) {
        return true;
    }
    !matches!(
        c,
        0xd3 | 0xdb | 0xdd | 0xe3 | 0xe4 | 0xeb | 0xec | 0xed | 0xf4 | 0xfc | 0xfd
    )
}

fn pc8(env: &mut Environment) -> u8 {
    env.advance_pc()
}
fn pc16(env: &mut Environment) -> u16 {
    env.advance_immediate16()
}
fn addr(r: u16) -> u32 {
    r as u32
}
fn read8(env: &Environment, i: usize) -> u8 {
    if i == 6 {
        env.peek(addr(env.state.reg.get16(Reg16::HL)))
    } else {
        env.state.reg.get8(R8[i])
    }
}
fn write8(env: &mut Environment, i: usize, v: u8) {
    if i == 6 {
        env.poke(addr(env.state.reg.get16(Reg16::HL)), v);
    } else {
        env.state.reg.set8(R8[i], v);
    }
}
fn flags(env: &mut Environment, v: u8) {
    env.state.reg.set8(Reg8::F, v & 0xf0);
}
fn f(env: &Environment) -> u8 {
    env.state.reg.get8(Reg8::F)
}
fn cond(env: &Environment, n: usize) -> bool {
    let bit = if n < 2 { Z } else { C };
    (f(env) & bit != 0) == (n & 1 != 0)
}
fn push(env: &mut Environment, v: u16) {
    let sp = env.state.reg.get16(Reg16::SP).wrapping_sub(1);
    env.state.reg.set16(Reg16::SP, sp);
    env.poke(addr(sp), (v >> 8) as u8);
    let sp = sp.wrapping_sub(1);
    env.state.reg.set16(Reg16::SP, sp);
    env.poke(addr(sp), v as u8);
}
fn pop(env: &mut Environment) -> u16 {
    let sp = env.state.reg.get16(Reg16::SP);
    let lo = env.peek(addr(sp));
    let sp = sp.wrapping_add(1);
    let hi = env.peek(addr(sp));
    env.state.reg.set16(Reg16::SP, sp.wrapping_add(1));
    u16::from_le_bytes([lo, hi])
}
fn jr(env: &mut Environment) {
    let d = pc8(env) as i8;
    let pc = env.state.pc() as u16;
    env.state.set_pc(pc.wrapping_add(d as i16 as u16) as u32);
}

fn inc(env: &mut Environment, i: usize) {
    let a = read8(env, i);
    let v = a.wrapping_add(1);
    write8(env, i, v);
    flags(
        env,
        (if v == 0 { Z } else { 0 }) | if a & 0x0f == 0x0f { H } else { 0 } | (f(env) & C),
    );
}
fn dec(env: &mut Environment, i: usize) {
    let a = read8(env, i);
    let v = a.wrapping_sub(1);
    write8(env, i, v);
    flags(
        env,
        (if v == 0 { Z } else { 0 }) | N | if a & 0x0f == 0 { H } else { 0 } | (f(env) & C),
    );
}
fn alu(env: &mut Environment, op: usize, b: u8) {
    let a = env.state.reg.a();
    let carry = if f(env) & C != 0 { 1 } else { 0 };
    let (v, fl) = match op {
        0 => {
            let (v, c) = a.overflowing_add(b);
            (
                v,
                (if v == 0 { Z } else { 0 })
                    | if (a & 15) + (b & 15) > 15 { H } else { 0 }
                    | if c { C } else { 0 },
            )
        }
        1 => {
            let (x, c1) = a.overflowing_add(b);
            let (v, c2) = x.overflowing_add(carry);
            (
                v,
                (if v == 0 { Z } else { 0 })
                    | if (a & 15) + (b & 15) + carry > 15 {
                        H
                    } else {
                        0
                    }
                    | if c1 || c2 { C } else { 0 },
            )
        }
        2 => {
            let (v, c) = a.overflowing_sub(b);
            (
                v,
                (if v == 0 { Z } else { 0 })
                    | N
                    | if (a & 15) < (b & 15) { H } else { 0 }
                    | if c { C } else { 0 },
            )
        }
        3 => {
            let (x, c1) = a.overflowing_sub(b);
            let (v, c2) = x.overflowing_sub(carry);
            (
                v,
                (if v == 0 { Z } else { 0 })
                    | N
                    | if (a & 15) < (b & 15) + carry { H } else { 0 }
                    | if c1 || c2 { C } else { 0 },
            )
        }
        4 => (a & b, if a & b == 0 { Z | H } else { H }),
        5 => (a ^ b, if a ^ b == 0 { Z } else { 0 }),
        6 => (a | b, if a | b == 0 { Z } else { 0 }),
        _ => {
            let (_, c) = a.overflowing_sub(b);
            (
                a,
                (if a == b { Z } else { 0 })
                    | N
                    | if (a & 15) < (b & 15) { H } else { 0 }
                    | if c { C } else { 0 },
            )
        }
    };
    flags(env, fl);
    if op != 7 {
        env.state.reg.set_a(v);
    }
}

fn execute_base(env: &mut Environment, c: u8) {
    if (0x40..=0x7f).contains(&c) {
        if c == 0x76 {
            env.state.halted = true;
        } else {
            write8(env, ((c >> 3) & 7) as usize, read8(env, (c & 7) as usize));
        }
        return;
    }
    if (0x80..=0xbf).contains(&c) {
        alu(env, ((c >> 3) & 7) as usize, read8(env, (c & 7) as usize));
        return;
    }
    match c {
        0x00 => {}
        0x01 | 0x11 | 0x21 | 0x31 => {
            let value = pc16(env);
            env.state.reg.set16(RR[((c >> 4) & 3) as usize], value);
        }
        0x02 | 0x12 => env.poke(
            addr(env.state.reg.get16(RR[((c >> 4) & 1) as usize])),
            env.state.reg.a(),
        ),
        0x0a | 0x1a => {
            let v = env.peek(addr(env.state.reg.get16(RR[((c >> 4) & 1) as usize])));
            env.state.reg.set_a(v);
        }
        0x03 | 0x13 | 0x23 | 0x33 => {
            let r = RR[((c >> 4) & 3) as usize];
            env.state
                .reg
                .set16(r, env.state.reg.get16(r).wrapping_add(1));
        }
        0x0b | 0x1b | 0x2b | 0x3b => {
            let r = RR[((c >> 4) & 3) as usize];
            env.state
                .reg
                .set16(r, env.state.reg.get16(r).wrapping_sub(1));
        }
        0x04 | 0x0c | 0x14 | 0x1c | 0x24 | 0x2c | 0x34 | 0x3c => inc(env, ((c >> 3) & 7) as usize),
        0x05 | 0x0d | 0x15 | 0x1d | 0x25 | 0x2d | 0x35 | 0x3d => dec(env, ((c >> 3) & 7) as usize),
        0x06 | 0x0e | 0x16 | 0x1e | 0x26 | 0x2e | 0x36 | 0x3e => {
            let v = pc8(env);
            write8(env, ((c >> 3) & 7) as usize, v);
        }
        0x07 | 0x0f | 0x17 | 0x1f => {
            let a = env.state.reg.a();
            let old = if f(env) & C != 0 { 1 } else { 0 };
            let (v, carry) = match c {
                0x07 => (a.rotate_left(1), a >> 7),
                0x0f => (a.rotate_right(1), a & 1),
                0x17 => ((a << 1) | old, a >> 7),
                _ => ((a >> 1) | (old << 7), a & 1),
            };
            env.state.reg.set_a(v);
            flags(env, if carry != 0 { C } else { 0 });
        }
        0x08 => {
            let a = pc16(env);
            let sp = env.state.reg.get16(Reg16::SP);
            env.poke(addr(a), sp as u8);
            env.poke(addr(a.wrapping_add(1)), (sp >> 8) as u8);
        }
        0x09 | 0x19 | 0x29 | 0x39 => {
            let a = env.state.reg.get16(Reg16::HL);
            let b = env.state.reg.get16(RR[((c >> 4) & 3) as usize]);
            let (v, carry) = a.overflowing_add(b);
            env.state.reg.set16(Reg16::HL, v);
            flags(
                env,
                (f(env) & Z)
                    | if (a & 0xfff) + (b & 0xfff) > 0xfff {
                        H
                    } else {
                        0
                    }
                    | if carry { C } else { 0 },
            );
        }
        0x10 => {
            pc8(env);
            env.state.halted = true;
        }
        0x18 => jr(env),
        0x20 | 0x28 | 0x30 | 0x38 => {
            if cond(env, ((c >> 3) & 3) as usize) {
                jr(env)
            } else {
                pc8(env);
            }
        }
        0x22 | 0x32 => {
            let h = env.state.reg.get16(Reg16::HL);
            env.poke(addr(h), env.state.reg.a());
            env.state.reg.set16(
                Reg16::HL,
                if c == 0x22 {
                    h.wrapping_add(1)
                } else {
                    h.wrapping_sub(1)
                },
            );
        }
        0x2a | 0x3a => {
            let h = env.state.reg.get16(Reg16::HL);
            env.state.reg.set_a(env.peek(addr(h)));
            env.state.reg.set16(
                Reg16::HL,
                if c == 0x2a {
                    h.wrapping_add(1)
                } else {
                    h.wrapping_sub(1)
                },
            );
        }
        0x27 => {
            let a = env.state.reg.a();
            let old = f(env);
            let mut adjust = 0;
            let mut carry = old & C != 0;
            if old & N == 0 {
                if old & H != 0 || a & 15 > 9 {
                    adjust |= 6
                };
                if carry || a > 0x99 {
                    adjust |= 0x60;
                    carry = true
                }
            } else {
                if old & H != 0 {
                    adjust |= 6
                };
                if carry {
                    adjust |= 0x60
                }
            };
            let v = if old & N == 0 {
                a.wrapping_add(adjust)
            } else {
                a.wrapping_sub(adjust)
            };
            env.state.reg.set_a(v);
            flags(
                env,
                (if v == 0 { Z } else { 0 }) | (old & N) | if carry { C } else { 0 },
            );
        }
        0x2f => {
            env.state.reg.set_a(!env.state.reg.a());
            flags(env, (f(env) & (Z | C)) | N | H);
        }
        0x37 => flags(env, (f(env) & Z) | C),
        0x3f => flags(env, (f(env) & Z) | if f(env) & C == 0 { C } else { 0 }),
        0xc0 | 0xc8 | 0xd0 | 0xd8 => {
            if cond(env, ((c >> 3) & 3) as usize) {
                let address = pop(env);
                env.state.set_pc(address as u32);
            }
        }
        0xc1 | 0xd1 | 0xe1 | 0xf1 => {
            let r = [Reg16::BC, Reg16::DE, Reg16::HL, Reg16::AF][((c >> 4) & 3) as usize];
            let v = pop(env);
            env.state
                .reg
                .set16(r, if r == Reg16::AF { v & 0xfff0 } else { v });
        }
        0xc2 | 0xca | 0xd2 | 0xda => {
            let a = pc16(env);
            if cond(env, ((c >> 3) & 3) as usize) {
                env.state.set_pc(a as u32)
            }
        }
        0xc3 => {
            let address = pc16(env);
            env.state.set_pc(address as u32);
        }
        0xc4 | 0xcc | 0xd4 | 0xdc => {
            let a = pc16(env);
            if cond(env, ((c >> 3) & 3) as usize) {
                push(env, env.state.pc() as u16);
                env.state.set_pc(a as u32)
            }
        }
        0xc5 | 0xd5 | 0xe5 | 0xf5 => push(
            env,
            env.state
                .reg
                .get16([Reg16::BC, Reg16::DE, Reg16::HL, Reg16::AF][((c >> 4) & 3) as usize]),
        ),
        0xc6 | 0xce | 0xd6 | 0xde | 0xe6 | 0xee | 0xf6 | 0xfe => {
            let v = pc8(env);
            alu(env, ((c >> 3) & 7) as usize, v);
        }
        0xc7 | 0xcf | 0xd7 | 0xdf | 0xe7 | 0xef | 0xf7 | 0xff => {
            push(env, env.state.pc() as u16);
            env.state.set_pc((c & 0x38) as u32);
        }
        0xc9 => {
            let address = pop(env);
            env.state.set_pc(address as u32);
        }
        0xcd => {
            let a = pc16(env);
            push(env, env.state.pc() as u16);
            env.state.set_pc(a as u32);
        }
        0xd9 => {
            let address = pop(env);
            env.state.set_pc(address as u32);
            env.state.reg.iff1 = true;
            env.state.reg.iff2 = true;
        }
        0xe0 => {
            let a = 0xff00u16 | pc8(env) as u16;
            env.poke(addr(a), env.state.reg.a());
        }
        0xe2 => {
            let a = 0xff00u16 | env.state.reg.get8(Reg8::C) as u16;
            env.poke(addr(a), env.state.reg.a());
        }
        0xea => {
            let a = pc16(env);
            env.poke(addr(a), env.state.reg.a());
        }
        0xe8 | 0xf8 => {
            let sp = env.state.reg.get16(Reg16::SP);
            let d = pc8(env) as i8 as i16 as u16;
            let v = sp.wrapping_add(d);
            let fl = if (sp & 15) + (d & 15) > 15 { H } else { 0 }
                | if (sp & 0xff) + (d & 0xff) > 0xff {
                    C
                } else {
                    0
                };
            if c == 0xe8 {
                env.state.reg.set16(Reg16::SP, v)
            } else {
                env.state.reg.set16(Reg16::HL, v)
            };
            flags(env, fl);
        }
        0xe9 => env.state.set_pc(env.state.reg.get16(Reg16::HL) as u32),
        0xf0 => {
            let a = 0xff00u16 | pc8(env) as u16;
            env.state.reg.set_a(env.peek(addr(a)));
        }
        0xf2 => {
            let a = 0xff00u16 | env.state.reg.get8(Reg8::C) as u16;
            env.state.reg.set_a(env.peek(addr(a)));
        }
        0xfa => {
            let a = pc16(env);
            env.state.reg.set_a(env.peek(addr(a)));
        }
        0xf3 => {
            env.state.reg.iff1 = false;
            env.state.reg.iff2 = false;
        }
        0xfb => {
            env.state.reg.iff1 = true;
            env.state.reg.iff2 = true;
        }
        0xf9 => env
            .state
            .reg
            .set16(Reg16::SP, env.state.reg.get16(Reg16::HL)),
        _ => unreachable!(),
    }
}

fn execute_cb(env: &mut Environment, c: u8) {
    let i = (c & 7) as usize;
    let group = (c >> 3) as usize;
    let a = read8(env, i);
    if group < 8 {
        let old = if f(env) & C != 0 { 1 } else { 0 };
        let (v, carry) = match group {
            0 => (a.rotate_left(1), a >> 7),
            1 => (a.rotate_right(1), a & 1),
            2 => ((a << 1) | old, a >> 7),
            3 => ((a >> 1) | (old << 7), a & 1),
            4 => (a << 1, a >> 7),
            5 => (((a as i8) >> 1) as u8, a & 1),
            6 => ((a << 4) | (a >> 4), 0),
            _ => (a >> 1, a & 1),
        };
        write8(env, i, v);
        flags(
            env,
            (if v == 0 { Z } else { 0 }) | if carry != 0 { C } else { 0 },
        );
    } else if group < 16 {
        flags(
            env,
            (f(env) & C) | H | if a & (1 << (group - 8)) == 0 { Z } else { 0 },
        );
    } else if group < 24 {
        write8(env, i, a & !(1 << (group - 16)));
    } else {
        write8(env, i, a | (1 << (group - 24)));
    }
}
