use super::environment::*;
use super::opcode::*;
use super::operators::*;
use super::registers::*;

pub fn build_lea_rr_ind_offset(dest: Reg16, src: Reg16) -> Opcode {
    Opcode {
        name: format!("LEA {:?}, {:?}d", dest, src),
        action: Box::new(move |env: &mut Environment| {
            let imm = env.advance_pc() as i8 as i32 as u32;
            if env.state.is_op_long() {
                let value = env.state.reg.get24(src).wrapping_add(imm);
                env.state.reg.set24(dest, value);
            } else {
                let value = env.state.reg.get16(src).wrapping_add(imm as u16);
                env.state.reg.set16(dest, value);
            }
        }),
    }
}

pub fn build_pea(src: Reg16) -> Opcode {
    Opcode {
        name: format!("PEA {:?}d", src),
        action: Box::new(move |env: &mut Environment| {
            let imm = env.advance_pc() as i8 as i32 as u32;
            if env.state.is_op_long() {
                let value = env.state.reg.get24(src).wrapping_add(imm);
                env.push(value);
            } else {
                let value = env.state.reg.get16(src).wrapping_add(imm as u16);
                env.push(value as u32);
            }
        }),
    }
}

pub fn build_tst_a_r(reg: Reg8) -> Opcode {
    Opcode {
        name: format!("TST A, {}", reg),
        action: Box::new(move |env: &mut Environment| {
            let a = env.state.reg.a();
            let b = env.reg8_ext(reg);
            operator_tst(env, a, b);
        }),
    }
}

pub fn build_tst_a_n() -> Opcode {
    Opcode {
        name: format!("TST A, n"),
        action: Box::new(move |env: &mut Environment| {
            let a = env.state.reg.a();
            let b = env.advance_pc();
            operator_tst(env, a, b);
        }),
    }
}

pub fn build_swapnib() -> Opcode {
    Opcode {
        name: "SWAPNIB".to_string(),
        action: Box::new(move |env: &mut Environment| {
            let a = env.state.reg.a();
            env.state.reg.set_a(a.rotate_left(4));
        }),
    }
}

pub fn build_mirror_a() -> Opcode {
    Opcode {
        name: "MIRROR A".to_string(),
        action: Box::new(move |env: &mut Environment| {
            let a = env.state.reg.a();
            env.state.reg.set_a(a.reverse_bits());
        }),
    }
}

pub fn build_mul_de() -> Opcode {
    Opcode {
        name: "MUL D, E".to_string(),
        action: Box::new(move |env: &mut Environment| {
            let d = env.state.reg.get8(Reg8::D) as u16;
            let e = env.state.reg.get8(Reg8::E) as u16;
            env.state.reg.set16(Reg16::DE, d * e);
        }),
    }
}

pub fn build_bs_de_b(mode: NextBsMode) -> Opcode {
    Opcode {
        name: mode.name().to_string(),
        action: Box::new(move |env: &mut Environment| {
            let mut value = env.state.reg.get16(Reg16::DE);
            for _ in 0..env.state.reg.get8(Reg8::B) {
                value = match mode {
                    NextBsMode::LeftArithmetic => value.wrapping_shl(1),
                    NextBsMode::RightArithmetic => ((value as i16) >> 1) as u16,
                    NextBsMode::RightLogical => value >> 1,
                    NextBsMode::RightFill => (value >> 1) | 0x8000,
                    NextBsMode::RotateLeft => value.rotate_left(1),
                };
            }
            env.state.reg.set16(Reg16::DE, value);
        }),
    }
}

#[derive(Copy, Clone)]
pub enum NextBsMode {
    LeftArithmetic,
    RightArithmetic,
    RightLogical,
    RightFill,
    RotateLeft,
}

impl NextBsMode {
    fn name(self) -> &'static str {
        match self {
            Self::LeftArithmetic => "BSLA DE, B",
            Self::RightArithmetic => "BSRA DE, B",
            Self::RightLogical => "BSRL DE, B",
            Self::RightFill => "BSRF DE, B",
            Self::RotateLeft => "BRLC DE, B",
        }
    }
}

pub fn build_add_rr_a(rr: Reg16) -> Opcode {
    Opcode {
        name: format!("ADD {:?}, A", rr),
        action: Box::new(move |env: &mut Environment| {
            let value = env
                .state
                .reg
                .get16(rr)
                .wrapping_add(env.state.reg.a() as u16);
            env.state.reg.set16(rr, value);
        }),
    }
}

pub fn build_add_rr_nn(rr: Reg16) -> Opcode {
    Opcode {
        name: format!("ADD {:?}, nn", rr),
        action: Box::new(move |env: &mut Environment| {
            let lo = env.advance_pc() as u16;
            let hi = env.advance_pc() as u16;
            let value = env.state.reg.get16(rr).wrapping_add(lo | (hi << 8));
            env.state.reg.set16(rr, value);
        }),
    }
}

pub fn build_operator_a_idx_offset(idx: Reg16, (op, name): (Operator, &str)) -> Opcode {
    Opcode {
        name: format!("{} A, ({:?}d)", name, idx),
        action: Box::new(move |env: &mut Environment| {
            let offset = env.advance_pc() as i8 as i32 as u32;
            let a = env.state.reg.a();
            let address = if env.state.is_op_long() {
                env.state.reg.get24(idx).wrapping_add(offset)
            } else {
                env.state.reg.get16_mbase_offset(idx, offset as u16)
            };
            let b = env.peek(address);
            let v = op(env, a, b);
            env.state.reg.set_a(v);
        }),
    }
}

pub fn build_operator_a_r(r: Reg8, (op, name): (Operator, &str)) -> Opcode {
    if r != Reg8::_HL && r != Reg8::H && r != Reg8::L {
        // Fast version
        Opcode {
            name: format!("{} A, {}", name, r),
            action: Box::new(move |env: &mut Environment| {
                let a = env.state.reg.a();
                let b = env.state.reg.get8(r);
                let v = op(env, a, b);
                env.state.reg.set_a(v);
            }),
        }
    } else {
        Opcode {
            name: format!("{} A, {}", name, r),
            action: Box::new(move |env: &mut Environment| {
                let a = env.state.reg.a();
                let b = env.reg8_ext(r);
                let v = op(env, a, b);

                env.state.reg.set_a(v);
            }),
        }
    }
}

pub fn build_operator_a_n((op, name): (Operator, &str)) -> Opcode {
    Opcode {
        name: format!("{} A, n", name),
        action: Box::new(move |env: &mut Environment| {
            let a = env.state.reg.a();
            let b = env.advance_pc();
            let v = op(env, a, b);

            env.state.reg.set_a(v);
        }),
    }
}

pub fn build_cp_block((inc, repeat, postfix): (bool, bool, &'static str)) -> Opcode {
    Opcode {
        name: format!("CP{}", postfix),
        action: Box::new(move |env: &mut Environment| {
            let a = env.state.reg.a();
            let b = env.reg8_ext(Reg8::_HL);
            let c_bak = env.state.reg.get_flag(Flag::C);
            operator_cp(env, a, b);
            let memptr = env
                .state
                .reg
                .memptr
                .wrapping_add(if inc { 1 } else { 0xffff });
            let bc = if env.state.is_op_long() {
                env.state.reg.inc_dec24(Reg16::HL, inc);
                env.state.reg.inc_dec24(Reg16::BC, false /*decrement*/)
            } else {
                env.state.reg.inc_dec16(Reg16::HL, inc);
                env.state.reg.inc_dec16(Reg16::BC, false /*decrement*/)
            };

            // TUZD-4.2
            let mut n = a.wrapping_sub(b);
            if env.state.reg.get_flag(Flag::H) {
                n = n.wrapping_sub(1);
            }
            env.state.reg.update_undocumented_flags_block(n);
            env.state.reg.set_flag(Flag::N);
            env.state.reg.put_flag(Flag::P, bc != 0);
            env.state.reg.put_flag(Flag::C, c_bak); // C unchanged
                                                    // S, Z and H set by operator_cp()
            env.state.reg.set_memptr(memptr);

            if repeat && bc != 0 && a != b {
                // Back to redo the instruction
                let pc = env.wrap_address(env.state.pc(), -2);
                env.state.set_pc(pc);
                env.state.reg.set_memptr((pc as u16).wrapping_add(1));
            }
        }),
    }
}

pub fn build_mlt_rr(reg: Reg16) -> Opcode {
    Opcode {
        name: format!("MLT {:?}", reg),
        action: Box::new(move |env: &mut Environment| {
            let r = env.state.reg.get16(reg);
            let a = r & 0xff;
            let b = (r >> 8) & 0xff;
            env.state.reg.set16(reg, a * b);
            env.sys.use_cycles(4);
        }),
    }
}
