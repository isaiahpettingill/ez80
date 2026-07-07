use super::environment::*;
use super::opcode::*;
use super::operators::*;
use super::registers::*;

/*
    From "The undocumented Z80 documented" TUZD-4.4:

Officially the Z80 has an 8 bit I/O port address space. When using the I/O ports, the 16 address
lines are used. And in fact, the high 8 bit do actually have some value, so you can use 65536
ports after all. IN r,(C), OUT (C),r, and the Block I/O instructions actually place the entire BC
register on the address bus. Similarly IN A,(n) and OUT (n),A put A × 256 + n on the address
bus.
The INI/INIR/IND/INDR instructions use BC after decrementing B, and the OUTI/OTIR/OUTD/OTDR
instructions before.
*/

pub fn build_out_c_r(r: Reg8) -> Opcode {
    Opcode {
        name: format!("OUT (C), {}", r),
        action: Box::new(move |env: &mut Environment| {
            let address = env.state.reg.get16(Reg16::BC);
            let value = env.state.reg.get8(r);
            env.port_out(address, value);
            env.state.reg.set_memptr(address.wrapping_add(1));
        }),
    }
}

pub fn build_out_c_0() -> Opcode {
    Opcode {
        name: "OUT (C), 0".to_string(),
        action: Box::new(move |env: &mut Environment| {
            let address = env.state.reg.get16(Reg16::BC);
            env.port_out(address, 0);
            env.state.reg.set_memptr(address.wrapping_add(1));
        }),
    }
}

pub fn build_out_n_a() -> Opcode {
    Opcode {
        name: "OUT (n), A".to_string(),
        action: Box::new(move |env: &mut Environment| {
            let a = env.state.reg.a();
            let port = env.advance_pc();
            let address = ((a as u16) << 8) + port as u16;
            env.port_out(address, a);
            env.state
                .reg
                .set_memptr(((a as u16) << 8) | port.wrapping_add(1) as u16);
        }),
    }
}

pub fn build_out0_n_r(r: Reg8) -> Opcode {
    Opcode {
        name: format!("OUT0 (n), {}", r),
        action: Box::new(move |env: &mut Environment| {
            let address = env.advance_pc() as u16;
            let data = env.state.reg.get8(r);
            env.port_out(address, data);
        }),
    }
}

pub fn build_in0_r_n(r: Reg8) -> Opcode {
    Opcode {
        name: format!("IN0 {}, (n)", r),
        action: Box::new(move |env: &mut Environment| {
            let address = env.advance_pc() as u16;
            let data = env.port_in(address);
            env.state.reg.update_arithmetic_flags(
                data as u16,
                data as u16,
                data as u16,
                true,
                false,
            );
            env.state.reg.set8(r, data);
        }),
    }
}

pub fn build_in_r_c(r: Reg8) -> Opcode {
    Opcode {
        name: format!("IN {}, (C)", r),
        action: Box::new(move |env: &mut Environment| {
            let address = env.state.reg.get16(Reg16::BC);
            let value = env.port_in(address);
            env.state.reg.set8(r, value);
            env.state.reg.set_memptr(address.wrapping_add(1));

            env.state.reg.update_bits_in_flags(value);
        }),
    }
}

pub fn build_in_0_c() -> Opcode {
    Opcode {
        name: "IN (C)".to_string(),
        action: Box::new(move |env: &mut Environment| {
            let address = env.state.reg.get16(Reg16::BC);
            let value = env.port_in(address);
            env.state.reg.set_memptr(address.wrapping_add(1));

            env.state.reg.update_bits_in_flags(value);
        }),
    }
}

pub fn build_in_a_n() -> Opcode {
    Opcode {
        name: "IN A, (n)".to_string(),
        action: Box::new(move |env: &mut Environment| {
            let a = env.state.reg.a();
            let address = ((a as u16) << 8) + env.advance_pc() as u16;
            let value = env.port_in(address);
            env.state.reg.set_memptr(address.wrapping_add(1));
            env.state.reg.set_a(value);
        }),
    }
}

pub fn build_tstio_n() -> Opcode {
    Opcode {
        name: "TSTIO n".to_string(),
        action: Box::new(move |env: &mut Environment| {
            let value = env.port_in(env.state.reg.get8(Reg8::C) as u16);
            let mask = env.advance_pc();
            operator_tst(env, value, mask);
        }),
    }
}

pub fn build_nextreg_n_a() -> Opcode {
    Opcode {
        name: "NEXTREG n, A".to_string(),
        action: Box::new(move |env: &mut Environment| {
            let register = env.advance_pc();
            let value = env.state.reg.a();
            env.port_out(0x243b, register);
            env.port_out(0x253b, value);
        }),
    }
}

pub fn build_nextreg_n_n() -> Opcode {
    Opcode {
        name: "NEXTREG n, n".to_string(),
        action: Box::new(move |env: &mut Environment| {
            let register = env.advance_pc();
            let value = env.advance_pc();
            env.port_out(0x243b, register);
            env.port_out(0x253b, value);
        }),
    }
}

fn inc_dec16or24(env: &mut Environment, rr: Reg16, inc: bool) -> u32 {
    if env.state.is_op_long() {
        env.state.reg.inc_dec24(rr, inc)
    } else {
        env.state.reg.inc_dec16(rr, inc)
    }
}

fn set_simple_block_io_flags(env: &mut Environment, counter: u32, value: u8) {
    env.state.reg.put_flag(Flag::Z, counter == 0);
    env.state.reg.put_flag(Flag::N, value & 0x80 != 0);
}

fn repeat_ed_instruction(env: &mut Environment, counter: u32) {
    if counter != 0 {
        let instruction_len = match env.state.sz_prefix {
            crate::state::SizePrefix::None => 2,
            _ => 3,
        };
        let pc = env.wrap_address(env.state.pc(), -instruction_len);
        env.state.set_pc(pc);
    }
}

pub fn build_in_block2(inc: bool, repeat: bool) -> Opcode {
    Opcode {
        name: format!(
            "IN{}2{}",
            if inc { 'I' } else { 'D' },
            if repeat { "R" } else { "" }
        ),
        action: Box::new(move |env: &mut Environment| {
            let address = if repeat {
                env.state.reg.get16(Reg16::DE)
            } else {
                env.state.reg.get16(Reg16::BC)
            };
            let value = env.port_in(address);
            env.set_reg(Reg8::_HL, value);

            let counter = if repeat {
                inc_dec16or24(env, Reg16::BC, false)
            } else {
                let b = env.state.reg.inc_dec8(Reg8::B, false) as u32;
                env.state.reg.inc_dec8(Reg8::C, inc);
                b
            };
            if repeat {
                inc_dec16or24(env, Reg16::DE, inc);
            }
            inc_dec16or24(env, Reg16::HL, inc);

            set_simple_block_io_flags(env, counter, value);
            if repeat {
                repeat_ed_instruction(env, counter);
            }
        }),
    }
}

pub fn build_out_block2(inc: bool, repeat: bool) -> Opcode {
    let prefix = if repeat { "OT" } else { "OUT" };
    Opcode {
        name: format!(
            "{}{}2{}",
            prefix,
            if inc { 'I' } else { 'D' },
            if repeat { "R" } else { "" }
        ),
        action: Box::new(move |env: &mut Environment| {
            let value = env.reg8_ext(Reg8::_HL);
            let address = if repeat {
                env.state.reg.get16(Reg16::DE)
            } else {
                env.state.reg.get16(Reg16::BC)
            };
            env.port_out(address, value);

            let counter = if repeat {
                inc_dec16or24(env, Reg16::BC, false)
            } else {
                let b = env.state.reg.inc_dec8(Reg8::B, false) as u32;
                env.state.reg.inc_dec8(Reg8::C, inc);
                b
            };
            if repeat {
                inc_dec16or24(env, Reg16::DE, inc);
            }
            inc_dec16or24(env, Reg16::HL, inc);

            set_simple_block_io_flags(env, counter, value);
            if repeat {
                repeat_ed_instruction(env, counter);
            }
        }),
    }
}

pub fn build_otim_otdm(inc: bool, repeat: bool) -> Opcode {
    Opcode {
        name: format!(
            "OT{}M{}",
            if inc { 'I' } else { 'D' },
            if repeat { "R" } else { "" }
        ),
        action: Box::new(move |env: &mut Environment| {
            let value = env.reg8_ext(Reg8::_HL);
            let address = env.state.reg.get8(Reg8::C) as u16;
            env.port_out(address, value);

            let b = env.state.reg.inc_dec8(Reg8::B, false) as u32;
            env.state.reg.inc_dec8(Reg8::C, inc);
            inc_dec16or24(env, Reg16::HL, inc);

            set_simple_block_io_flags(env, b, value);
            if repeat {
                repeat_ed_instruction(env, b);
            }
        }),
    }
}

pub fn build_inirx_or_indrx(inc: bool) -> Opcode {
    Opcode {
        name: format!("IN{}RX", if inc { 'I' } else { 'D' }),
        action: Box::new(move |env: &mut Environment| {
            let address = env.state.reg.get16(Reg16::DE);
            let value = env.port_in(address);
            env.set_reg(Reg8::_HL, value);

            let counter = inc_dec16or24(env, Reg16::BC, false);
            inc_dec16or24(env, Reg16::HL, inc);

            set_simple_block_io_flags(env, counter, value);
            repeat_ed_instruction(env, counter);
        }),
    }
}

/*
, and the OUTI/OTIR/OUTD/OTDR
instructions before.
*/

pub fn build_in_block((inc, repeat, postfix): (bool, bool, &'static str)) -> Opcode {
    Opcode {
        name: format!("IN{}", postfix),
        action: Box::new(move |env: &mut Environment| {
            let original_bc = env.state.reg.get16(Reg16::BC);
            let b = env.state.reg.inc_dec8(Reg8::B, false /* decrement */);
            let address = env.state.reg.get16(Reg16::BC);
            let value = env.port_in(address);
            env.set_reg(Reg8::_HL, value);
            inc_dec16or24(env, Reg16::HL, inc);
            env.state
                .reg
                .set_memptr(original_bc.wrapping_add(if inc { 1 } else { 0xffff }));

            let c = env.state.reg.get8(Reg8::C);
            let low = if inc {
                c.wrapping_add(1)
            } else {
                c.wrapping_sub(1)
            };
            let k = value as u16 + low as u16;
            env.state.reg.update_block_flags(value, k, b);

            if repeat && b != 0 {
                // Back to redo the instruction
                let pc = env.wrap_address(env.state.pc(), -2);
                env.state.set_pc(pc);
            }
        }),
    }
}

pub fn build_out_block((inc, repeat, postfix): (bool, bool, &'static str)) -> Opcode {
    let n0 = if repeat { "OT" } else { "OUT" };
    Opcode {
        name: format!("{}{}", n0, postfix),
        action: Box::new(move |env: &mut Environment| {
            // the OUTI/OTIR/OUTD/OTDR instructions use BC before decrementing B
            let address = env.state.reg.get16(Reg16::BC);
            let b = env.state.reg.inc_dec8(Reg8::B, false /* decrement */);
            let memptr = env
                .state
                .reg
                .get16(Reg16::BC)
                .wrapping_add(if inc { 1 } else { 0xffff });

            // We won't have IX and IY cases to consider
            let value = env.reg8_ext(Reg8::_HL);
            env.port_out(address, value);
            env.state.reg.set_memptr(memptr);
            if env.state.is_op_long() {
                env.state.reg.inc_dec24(Reg16::HL, inc);
            } else {
                env.state.reg.inc_dec16(Reg16::HL, inc);
            }

            // TUZD-4.3
            let k = value as u16 + env.state.reg.get8(Reg8::L) as u16;
            env.state.reg.update_block_flags(value, k, b);

            if repeat && b != 0 {
                // Back to redo the instruction
                let pc = env.wrap_address(env.state.pc(), -2);
                env.state.set_pc(pc);
            }
        }),
    }
}

pub fn build_otirx_or_otdrx(inc: bool) -> Opcode {
    Opcode {
        name: format!("OT{}RX", if inc { 'I' } else { 'D' }),
        action: Box::new(move |env: &mut Environment| {
            let value = env.reg8_ext(Reg8::_HL);
            let address = env.state.reg.get16(Reg16::DE);
            env.sys.use_cycles(1);

            let bc = if env.state.is_op_long() {
                env.state.reg.inc_dec24(Reg16::HL, inc);
                env.state.reg.inc_dec24(Reg16::BC, false /*decrement*/)
            } else {
                env.state.reg.inc_dec16(Reg16::HL, inc);
                env.state.reg.inc_dec16(Reg16::BC, false /*decrement*/)
            };

            if env.state.cached_instruction {
                env.sys.use_cycles(-2);
            }

            env.port_out(address, value);

            // TUZD-4.3
            env.state.reg.put_flag(Flag::Z, bc == 0);
            env.state
                .reg
                .put_flag(Flag::N, if value & 0x80 == 0x80 { true } else { false });

            if bc != 0 {
                // Back to redo the instruction
                let instruction_len = match env.state.sz_prefix {
                    crate::state::SizePrefix::None => 2,
                    _ => 3,
                };
                let pc = env.wrap_address(env.state.pc(), -instruction_len);
                env.state.set_pc(pc);
                // and the size prefix is cached if present
                if let crate::state::SizePrefix::None = env.state.sz_prefix {
                } else {
                    env.sys.use_cycles(-1);
                }
                env.state.cached_instruction = true;
            } else {
                env.state.cached_instruction = false;
            }
        }),
    }
}
