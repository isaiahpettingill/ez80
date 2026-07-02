use super::fast_bus::FastBus;
use super::fast_env::FastEnv;
use super::opcode_bits::{ShiftDir, ShiftMode};
use super::registers::*;
use super::state::SizePrefix;

const NMI_ADDRESS: u32 = 0x0066;
const RP: [Reg16; 4] = [Reg16::BC, Reg16::DE, Reg16::HL, Reg16::SP];
const RP2: [Reg16; 4] = [Reg16::BC, Reg16::DE, Reg16::HL, Reg16::AF];
const R: [Reg8; 8] = [
    Reg8::B,
    Reg8::C,
    Reg8::D,
    Reg8::E,
    Reg8::H,
    Reg8::L,
    Reg8::_HL,
    Reg8::A,
];
const IM: [u8; 8] = [0, 0, 1, 2, 0, 0, 1, 2];
const CC: [(Flag, bool); 8] = [
    (Flag::Z, false),
    (Flag::Z, true),
    (Flag::C, false),
    (Flag::C, true),
    (Flag::P, false),
    (Flag::P, true),
    (Flag::S, false),
    (Flag::S, true),
];
const ROT: [(ShiftDir, ShiftMode); 8] = [
    (ShiftDir::Left, ShiftMode::RotateCarry),
    (ShiftDir::Right, ShiftMode::RotateCarry),
    (ShiftDir::Left, ShiftMode::Rotate),
    (ShiftDir::Right, ShiftMode::Rotate),
    (ShiftDir::Left, ShiftMode::Arithmetic),
    (ShiftDir::Right, ShiftMode::Arithmetic),
    (ShiftDir::Left, ShiftMode::Logical),
    (ShiftDir::Right, ShiftMode::Logical),
];
const BLI_A: [(bool, bool); 4] = [(true, false), (false, false), (true, true), (false, true)];

#[derive(Copy, Clone)]
struct Parts {
    x: usize,
    y: usize,
    z: usize,
    p: usize,
    q: usize,
}

impl Parts {
    fn new(code: u8) -> Self {
        Self {
            x: (code >> 6) as usize,
            y: ((code >> 3) & 7) as usize,
            z: (code & 7) as usize,
            p: ((code >> 4) & 3) as usize,
            q: ((code >> 3) & 1) as usize,
        }
    }
}

pub(crate) fn step_ez80_fast<B: FastBus>(
    state: &mut super::state::State,
    cycles: &mut u64,
    bus: &mut B,
) {
    if state.halted && !state.nmi_pending && !state.reset_pending {
        return;
    }

    let mut env = FastEnv::new(state, bus, cycles);
    if env.state.reset_pending {
        env.state.reset_pending = false;
        env.state.nmi_pending = false;
        env.state.halted = false;
        env.state.set_pc(0x0000);
        env.state.reg.set8(Reg8::I, 0x00);
        env.state.reg.set8(Reg8::R, 0x00);
        env.state.reg.set_interrupts(false);
        env.state.reg.set_interrupt_mode(0);
    } else if env.state.nmi_pending {
        env.state.nmi_pending = false;
        env.state.halted = false;
        env.state.reg.start_nmi();
        let was_z80 = !env.state.reg.adl;
        let had_madl = env.state.reg.madl;
        env.push(env.state.pc());
        env.state.set_pc(NMI_ADDRESS);
        if had_madl && was_z80 {
            env.push_byte_spl(0x01);
            env.state.reg.adl = true;
        }
    }

    env.state.reg.begin_instruction_flags();
    execute_prefixed(&mut env);
    env.state.reg.finish_instruction_flags();
    env.clear_index();
    env.state.clear_sz_prefix();
    env.state.instructions_executed += 1;
    let r = env.state.reg.get8(Reg8::R).wrapping_add(1);
    env.state.reg.set8(Reg8::R, r);
    env.finish();
}

fn execute_prefixed<B: FastBus>(env: &mut FastEnv<B>) {
    let mut b0 = env.advance_pc();
    loop {
        match b0 {
            0x40 => env.state.sz_prefix = SizePrefix::SIS,
            0x49 => env.state.sz_prefix = SizePrefix::LIS,
            0x52 => env.state.sz_prefix = SizePrefix::SIL,
            0x5b => env.state.sz_prefix = SizePrefix::LIL,
            _ => break,
        }
        b0 = env.advance_pc();
    }
    loop {
        match b0 {
            0xdd => env.set_index(Reg16::IX),
            0xfd => env.set_index(Reg16::IY),
            _ => break,
        }
        b0 = env.advance_pc();
    }

    match b0 {
        0xcb => {
            if env.is_alt_index() {
                env.load_displacement();
                let op = env.advance_pc();
                execute_cb_indexed(env, op);
            } else {
                let op = env.advance_pc();
                execute_cb(env, op);
            }
        }
        0xed => {
            env.clear_index();
            let op = env.advance_pc();
            execute_ed(env, op);
        }
        0x0f | 0x1f | 0x2f | 0x07 | 0x17 | 0x27 | 0x31 | 0x37 | 0x3e | 0x3f | 0x86 | 0x96
        | 0xa6 | 0xb6 | 0x8e | 0x9e | 0xae | 0xbe
            if env.is_alt_index() =>
        {
            let index = env.get_index();
            env.clear_index();
            execute_dd_fd(env, index, b0);
        }
        _ => {
            if has_displacement(b0) && env.is_alt_index() {
                env.load_displacement();
            }
            execute_no_prefix(env, b0);
        }
    }
}

fn has_displacement(op: u8) -> bool {
    matches!(
        op,
        0x34 | 0x35
            | 0x36
            | 0x46
            | 0x4e
            | 0x56
            | 0x5e
            | 0x66
            | 0x6e
            | 0x70
            | 0x71
            | 0x72
            | 0x73
            | 0x74
            | 0x75
            | 0x77
            | 0x7e
            | 0x86
            | 0x8e
            | 0x96
            | 0x9e
            | 0xa6
            | 0xae
            | 0xb6
            | 0xbe
    )
}

fn execute_no_prefix<B: FastBus>(env: &mut FastEnv<B>, op: u8) {
    let p = Parts::new(op);
    match p.x {
        0 => match p.z {
            0 => match p.y {
                0 => {}
                1 => env.state.reg.swap16(Reg16::AF),
                2 => op_djnz(env),
                3 => op_jr(env),
                4..=7 => op_jr_cc(env, CC[p.y - 4]),
                _ => unreachable!(),
            },
            1 => match p.q {
                0 => op_ld_rr_nn(env, RP[p.p]),
                1 => op_add_hl_rr(env, RP[p.p]),
                _ => unreachable!(),
            },
            2 => match p.q {
                0 => match p.p {
                    0 => op_ld_prr_a(env, Reg16::BC),
                    1 => op_ld_prr_a(env, Reg16::DE),
                    2 => op_ld_pnn_rr(env, Reg16::HL),
                    3 => op_ld_pnn_a(env),
                    _ => unreachable!(),
                },
                1 => match p.p {
                    0 => op_ld_a_prr(env, Reg16::BC),
                    1 => op_ld_a_prr(env, Reg16::DE),
                    2 => op_ld_rr_pnn(env, Reg16::HL),
                    3 => op_ld_a_pnn(env),
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            },
            3 => op_inc_dec_rr(env, RP[p.p], p.q == 0),
            4 => op_inc_r(env, R[p.y]),
            5 => op_dec_r(env, R[p.y]),
            6 => op_ld_r_n(env, R[p.y]),
            7 => match p.y {
                0..=3 => op_rot_r(env, Reg8::A, ROT[p.y], true, false),
                4 => op_daa(env),
                5 => op_cpl(env),
                6 => op_scf(env),
                7 => op_ccf(env),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        },
        1 => {
            if p.z == 6 && p.y == 6 {
                env.state.halted = true;
            } else {
                op_ld_r_r(env, R[p.y], R[p.z]);
            }
        }
        2 => op_alu_a_r(env, p.y, R[p.z]),
        3 => match p.z {
            0 => op_ret_cc(env, CC[p.y]),
            1 => match p.q {
                0 => op_pop_rr(env, RP2[p.p]),
                1 => match p.p {
                    0 => op_ret(env),
                    1 => op_exx(env),
                    2 => op_jp_hl(env),
                    3 => op_ld_sp_hl(env),
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            },
            2 => op_jp_cc(env, CC[p.y]),
            3 => match p.y {
                0 => op_jp(env),
                1 => unreachable!(),
                2 => op_out_n_a(env),
                3 => op_in_a_n(env),
                4 => op_ex_psp_hl(env),
                5 => op_ex_de_hl(env),
                6 => env.state.reg.set_interrupts(false),
                7 => env.state.reg.set_interrupts(true),
                _ => unreachable!(),
            },
            4 => op_call_cc(env, CC[p.y]),
            5 => match p.q {
                0 => op_push_rr(env, RP2[p.p]),
                1 => match p.p {
                    0 => op_call(env),
                    1..=3 => unreachable!(),
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            },
            6 => op_alu_a_n(env, p.y),
            7 => op_rst(env, p.y as u8 * 8),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn execute_cb<B: FastBus>(env: &mut FastEnv<B>, op: u8) {
    let p = Parts::new(op);
    match p.x {
        0 => op_rot_r(env, R[p.z], ROT[p.y], false, false),
        1 => op_bit_r(env, p.y as u8, R[p.z]),
        2 => op_set_res_r(env, p.y as u8, R[p.z], false),
        3 => op_set_res_r(env, p.y as u8, R[p.z], true),
        _ => unreachable!(),
    }
}

fn execute_cb_indexed<B: FastBus>(env: &mut FastEnv<B>, op: u8) {
    let p = Parts::new(op);
    match p.x {
        0 => op_rot_r(env, R[p.z], ROT[p.y], false, true),
        1 => op_bit_r(env, p.y as u8, R[p.z]),
        2 => op_indexed_set_res_r(env, p.y as u8, R[p.z], false),
        3 => op_indexed_set_res_r(env, p.y as u8, R[p.z], true),
        _ => unreachable!(),
    }
}

fn execute_dd_fd<B: FastBus>(env: &mut FastEnv<B>, index: Reg16, op: u8) {
    match op {
        0x07 => op_ld_rr_idx_disp(env, Reg16::BC, index),
        0x0f => op_ld_idx_disp_rr(env, index, Reg16::BC),
        0x17 => op_ld_rr_idx_disp(env, Reg16::DE, index),
        0x1f => op_ld_idx_disp_rr(env, index, Reg16::DE),
        0x27 => op_ld_rr_idx_disp(env, Reg16::HL, index),
        0x2f => op_ld_idx_disp_rr(env, index, Reg16::HL),
        0x31 => op_ld_rr_idx_disp(
            env,
            if index == Reg16::IX {
                Reg16::IY
            } else {
                Reg16::IX
            },
            index,
        ),
        0x37 => op_ld_rr_idx_disp(env, index, index),
        0x3e => op_ld_idx_disp_rr(
            env,
            index,
            if index == Reg16::IX {
                Reg16::IY
            } else {
                Reg16::IX
            },
        ),
        0x3f => op_ld_idx_disp_rr(env, index, index),
        0x86 => op_alu_a_idx_offset(env, 0, index),
        0x8e => op_alu_a_idx_offset(env, 1, index),
        0x96 => op_alu_a_idx_offset(env, 2, index),
        0x9e => op_alu_a_idx_offset(env, 3, index),
        0xa6 => op_alu_a_idx_offset(env, 4, index),
        0xae => op_alu_a_idx_offset(env, 5, index),
        0xb6 => op_alu_a_idx_offset(env, 6, index),
        0xbe => op_alu_a_idx_offset(env, 7, index),
        _ => unreachable!(),
    }
}

fn execute_ed<B: FastBus>(env: &mut FastEnv<B>, op: u8) {
    if op == 0xfe {
        env.trap_illegal_instruction();
        return;
    }
    let p = Parts::new(op);
    match p.x {
        0 => match p.z {
            0 => match p.y {
                0 | 1 | 2 | 3 | 4 | 5 | 7 => op_in0_r_n(env, R[p.y]),
                _ => {}
            },
            1 => match p.y {
                6 => op_ld_rr_ind_hl(env, Reg16::IY),
                _ => op_out0_n_r(env, R[p.y]),
            },
            2 => match p.p {
                0 | 1 | 2 => op_lea_rr_ind_offset(env, RP[p.p], Reg16::IX),
                3 => op_lea_rr_ind_offset(env, Reg16::IX, Reg16::IX),
                _ => {}
            },
            3 => match p.p {
                0 | 1 | 2 => op_lea_rr_ind_offset(env, RP[p.p], Reg16::IY),
                3 => op_lea_rr_ind_offset(env, Reg16::IY, Reg16::IY),
                _ => {}
            },
            4 => op_tst_a_r(env, R[p.y]),
            6 => {
                if p.y == 7 {
                    op_ld_ind_hl_rr(env, Reg16::IY);
                }
            }
            7 => match p.y {
                0 | 2 | 4 => op_ld_rr_ind_hl(env, RP[p.p]),
                1 | 3 | 5 => op_ld_ind_hl_rr(env, RP[p.p]),
                6 => op_ld_rr_ind_hl(env, Reg16::IX),
                7 => op_ld_ind_hl_rr(env, Reg16::IX),
                _ => {}
            },
            _ => {}
        },
        1 => match p.z {
            0 => match p.y {
                6 => op_in_0_c(env),
                _ => op_in_r_c(env, R[p.y]),
            },
            1 => match p.y {
                6 => op_out_c_0(env),
                _ => op_out_c_r(env, R[p.y]),
            },
            2 => match p.q {
                0 => op_sbc_hl_rr(env, RP[p.p]),
                1 => op_adc_hl_rr(env, RP[p.p]),
                _ => unreachable!(),
            },
            3 => match p.q {
                0 => op_ld_pnn_rr(env, RP[p.p]),
                1 => op_ld_rr_pnn(env, RP[p.p]),
                _ => unreachable!(),
            },
            4 => match p.y {
                1 | 3 | 5 | 7 => op_mlt_rr(env, RP[p.p]),
                2 => op_lea_rr_ind_offset(env, Reg16::IX, Reg16::IY),
                4 => op_tst_a_n(env),
                6 => op_tstio_n(env),
                _ => op_neg(env),
            },
            5 => match p.y {
                1 => op_reti(env),
                2 => op_lea_rr_ind_offset(env, Reg16::IY, Reg16::IX),
                4 => op_pea(env, Reg16::IX),
                5 => env.state.reg.mbase = env.state.reg.get8(Reg8::A),
                7 => env.state.reg.madl = true,
                _ => op_retn(env),
            },
            6 => match p.y {
                4 => op_pea(env, Reg16::IY),
                5 => env.state.reg.set8(Reg8::A, env.state.reg.mbase),
                6 => env.state.halted = true,
                7 => env.state.reg.madl = false,
                _ => env.state.reg.set_interrupt_mode(IM[p.y]),
            },
            7 => match p.y {
                0 => op_ld_r_r(env, Reg8::I, Reg8::A),
                1 => op_ld_r_r(env, Reg8::R, Reg8::A),
                2 => op_ld_a_r_or_i(env, Reg8::I),
                3 => op_ld_a_r_or_i(env, Reg8::R),
                4 => op_rxd(env, ShiftDir::Right),
                5 => op_rxd(env, ShiftDir::Left),
                6 | 7 => {}
                _ => unreachable!(),
            },
            _ => unreachable!(),
        },
        2 => {
            if p.z <= 3 && p.y >= 4 {
                match p.z {
                    0 => op_ld_block(env, BLI_A[p.y - 4]),
                    1 => op_cp_block(env, BLI_A[p.y - 4]),
                    2 => op_in_block(env, BLI_A[p.y - 4]),
                    3 => op_out_block(env, BLI_A[p.y - 4]),
                    _ => unreachable!(),
                }
            } else if p.z == 3 {
                match p.y {
                    0 => op_otim_otdm(env, true, false),
                    1 => op_otim_otdm(env, false, false),
                    2 => op_otim_otdm(env, true, true),
                    3 => op_otim_otdm(env, false, true),
                    _ => {}
                }
            } else if p.z == 4 {
                match p.y {
                    0 => op_in_block2(env, true, false),
                    1 => op_in_block2(env, false, false),
                    2 => op_in_block2(env, true, true),
                    3 => op_in_block2(env, false, true),
                    4 => op_out_block2(env, true, false),
                    5 => op_out_block2(env, false, false),
                    6 => op_out_block2(env, true, true),
                    7 => op_out_block2(env, false, true),
                    _ => {}
                }
            }
        }
        3 => match p.z {
            2 => match p.y {
                0 => op_inirx_or_indrx(env, true),
                1 => op_inirx_or_indrx(env, false),
                _ => {}
            },
            3 => match p.y {
                0 => op_otirx_or_otdrx(env, true),
                1 => op_otirx_or_otdrx(env, false),
                _ => {}
            },
            7 => match p.y {
                0 => {
                    let value = env.state.reg.get16(Reg16::HL);
                    env.state.reg.set_i16(value);
                }
                2 => op_ld_hl_i(env),
                _ => {}
            },
            _ => {}
        },
        _ => unreachable!(),
    }
}

fn op_add<B: FastBus>(env: &mut FastEnv<B>, a: u8, b: u8) -> u8 {
    env.state.reg.clear_flag(Flag::C);
    op_adc(env, a, b)
}

fn op_adc<B: FastBus>(env: &mut FastEnv<B>, a: u8, b: u8) -> u8 {
    let aa = a as u16;
    let bb = b as u16;
    let mut vv = aa + bb;
    if env.state.reg.get_flag(Flag::C) {
        vv += 1;
    }
    env.state
        .reg
        .update_arithmetic_flags(aa, bb, vv, false, true);
    vv as u8
}

fn op_sub<B: FastBus>(env: &mut FastEnv<B>, a: u8, b: u8) -> u8 {
    env.state.reg.clear_flag(Flag::C);
    op_sbc(env, a, b)
}

fn op_sbc<B: FastBus>(env: &mut FastEnv<B>, a: u8, b: u8) -> u8 {
    let aa = a as u16;
    let bb = b as u16;
    let mut vv = aa.wrapping_sub(bb);
    if env.state.reg.get_flag(Flag::C) {
        vv = vv.wrapping_sub(1);
    }
    env.state
        .reg
        .update_arithmetic_flags(aa, bb, vv, true, true);
    vv as u8
}

fn op_and<B: FastBus>(env: &mut FastEnv<B>, a: u8, b: u8) -> u8 {
    let v = a & b;
    env.state.reg.update_logic_flags(a, b, v, true);
    v
}

fn op_xor<B: FastBus>(env: &mut FastEnv<B>, a: u8, b: u8) -> u8 {
    let v = a ^ b;
    env.state.reg.update_logic_flags(a, b, v, false);
    v
}

fn op_or<B: FastBus>(env: &mut FastEnv<B>, a: u8, b: u8) -> u8 {
    let v = a | b;
    env.state.reg.update_logic_flags(a, b, v, false);
    v
}

fn op_cp<B: FastBus>(env: &mut FastEnv<B>, a: u8, b: u8) -> u8 {
    env.state.reg.clear_flag(Flag::C);
    op_sbc(env, a, b);
    env.state.reg.update_undocumented_flags(b);
    a
}

fn op_alu<B: FastBus>(env: &mut FastEnv<B>, op: usize, a: u8, b: u8) -> u8 {
    match op {
        0 => op_add(env, a, b),
        1 => op_adc(env, a, b),
        2 => op_sub(env, a, b),
        3 => op_sbc(env, a, b),
        4 => op_and(env, a, b),
        5 => op_xor(env, a, b),
        6 => op_or(env, a, b),
        7 => op_cp(env, a, b),
        _ => unreachable!(),
    }
}

fn op_inc<B: FastBus>(env: &mut FastEnv<B>, a: u8) -> u8 {
    let aa = a as u16;
    let vv = aa + 1;
    env.state
        .reg
        .update_arithmetic_flags(aa, 0, vv, false, false);
    vv as u8
}

fn op_dec<B: FastBus>(env: &mut FastEnv<B>, a: u8) -> u8 {
    let aa = a as u16;
    let vv = aa.wrapping_sub(1);
    env.state
        .reg
        .update_arithmetic_flags(aa, 0, vv, true, false);
    vv as u8
}

fn op_add16<B: FastBus>(env: &mut FastEnv<B>, aa: u16, bb: u16) -> u16 {
    let aaaa = aa as u32;
    let bbbb = bb as u32;
    let vvvv = aaaa + bbbb;
    env.state.reg.update_add16_flags(aaaa, bbbb, vvvv);
    vvvv as u16
}

fn op_add24<B: FastBus>(env: &mut FastEnv<B>, a: u32, b: u32) -> u32 {
    let v = a + b;
    env.state.reg.update_add24_flags(a, b, v);
    v & 0xffffff
}

fn op_adc16<B: FastBus>(env: &mut FastEnv<B>, aa: u16, bb: u16) -> u16 {
    let aaaa = aa as u32;
    let bbbb = bb as u32;
    let mut vvvv = aaaa.wrapping_add(bbbb);
    if env.state.reg.get_flag(Flag::C) {
        vvvv = vvvv.wrapping_add(1);
    }
    let vv = vvvv as u16;
    env.state
        .reg
        .update_arithmetic_flags_16(aaaa, bbbb, vvvv, false);
    env.state.reg.put_flag(Flag::Z, vv == 0);
    vv
}

fn op_sbc16<B: FastBus>(env: &mut FastEnv<B>, aa: u16, bb: u16) -> u16 {
    let aaaa = aa as u32;
    let bbbb = bb as u32;
    let mut vvvv = aaaa.wrapping_sub(bbbb);
    if env.state.reg.get_flag(Flag::C) {
        vvvv = vvvv.wrapping_sub(1);
    }
    let vv = vvvv as u16;
    env.state
        .reg
        .update_arithmetic_flags_16(aaaa, bbbb, vvvv, true);
    env.state.reg.put_flag(Flag::Z, vv == 0);
    vv
}

fn op_adc24<B: FastBus>(env: &mut FastEnv<B>, a: u32, b: u32) -> u32 {
    let mut v = a.wrapping_add(b);
    if env.state.reg.get_flag(Flag::C) {
        v = v.wrapping_add(1);
    }
    let vv = v & 0xffffff;
    env.state.reg.update_arithmetic_flags_24(a, b, v, false);
    env.state.reg.put_flag(Flag::Z, vv == 0);
    vv
}

fn op_sbc24<B: FastBus>(env: &mut FastEnv<B>, a: u32, b: u32) -> u32 {
    let mut v = a.wrapping_sub(b);
    if env.state.reg.get_flag(Flag::C) {
        v = v.wrapping_sub(1);
    }
    let vv = v & 0xffffff;
    env.state.reg.update_arithmetic_flags_24(a, b, v, true);
    env.state.reg.put_flag(Flag::Z, vv == 0);
    vv
}

fn op_tst<B: FastBus>(env: &mut FastEnv<B>, a: u8, b: u8) {
    let v = a & b;
    env.state.reg.update_logic_flags(a, b, v, true);
}

fn inc_dec16or24<B: FastBus>(env: &mut FastEnv<B>, rr: Reg16, inc: bool) -> u32 {
    if env.state.is_op_long() {
        env.state.reg.inc_dec24(rr, inc)
    } else {
        env.state.reg.inc_dec16(rr, inc)
    }
}

fn op_ld_r_r<B: FastBus>(env: &mut FastEnv<B>, dst: Reg8, src: Reg8) {
    let value = if dst == Reg8::_HL {
        env.state.reg.get8(src)
    } else {
        env.reg8_ext(src)
    };
    if src == Reg8::_HL {
        env.state.reg.set8(dst, value);
    } else {
        env.set_reg(dst, value);
    }
}

fn op_ld_r_n<B: FastBus>(env: &mut FastEnv<B>, reg: Reg8) {
    let value = env.advance_pc();
    env.set_reg(reg, value);
}

fn op_ld_a_r_or_i<B: FastBus>(env: &mut FastEnv<B>, src: Reg8) {
    let value = if src == Reg8::R {
        let r = env.state.reg.get8(Reg8::R);
        (r & 0x80) | (r.wrapping_add(1) & 0x7f)
    } else {
        env.state.reg.get8(src)
    };
    env.state.reg.set8(Reg8::A, value);
    env.state.reg.put_flag(Flag::N, false);
    env.state.reg.put_flag(Flag::H, false);
    env.state.reg.put_flag(Flag::Z, value == 0);
    env.state.reg.put_flag(Flag::S, (value as i8) < 0);
    env.state.reg.put_flag(Flag::P, env.state.reg.iff2);
    env.state.reg.update_undocumented_flags(value);
}

fn op_ld_hl_i<B: FastBus>(env: &mut FastEnv<B>) {
    let value = env.state.reg.get_i16();
    env.state.reg.set16(Reg16::HL, value);
    env.state.reg.put_flag(Flag::N, false);
    env.state.reg.put_flag(Flag::H, false);
    env.state.reg.put_flag(Flag::Z, value == 0);
    env.state.reg.put_flag(Flag::S, (value & 0x8000) != 0);
    env.state.reg.put_flag(Flag::P, env.state.reg.iff2);
}

fn op_ld_rr_nn<B: FastBus>(env: &mut FastEnv<B>, rr: Reg16) {
    let value = env.advance_immediate16or24();
    env.set_reg16or24(rr, value);
}

fn op_ld_a_prr<B: FastBus>(env: &mut FastEnv<B>, rr: Reg16) {
    let address = env.reg16mbase_or_24(rr);
    let value = env.read8(address);
    env.state.reg.set_a(value);
}

fn op_ld_prr_a<B: FastBus>(env: &mut FastEnv<B>, rr: Reg16) {
    let address = env.reg16mbase_or_24(rr);
    env.write8(address, env.state.reg.a());
}

fn op_ld_a_pnn<B: FastBus>(env: &mut FastEnv<B>) {
    let address = env.advance_immediate_16mbase_or_24();
    let value = env.read8(address);
    env.state.reg.set_a(value);
}

fn op_ld_pnn_a<B: FastBus>(env: &mut FastEnv<B>) {
    let value = env.state.reg.a();
    let address = env.advance_immediate_16mbase_or_24();
    env.write8(address, value);
}

fn op_ld_pnn_rr<B: FastBus>(env: &mut FastEnv<B>, rr: Reg16) {
    let address = env.advance_immediate_16mbase_or_24();
    let value = env.reg16or24_ext(rr);
    if env.state.is_op_long() {
        env.write24(address, value);
    } else {
        env.write16(address, value as u16);
    }
}

fn op_ld_rr_pnn<B: FastBus>(env: &mut FastEnv<B>, rr: Reg16) {
    let address = env.advance_immediate_16mbase_or_24();
    if env.state.is_op_long() {
        let value = env.read24(address);
        env.set_reg24(rr, value);
    } else {
        let value = env.read16(address);
        env.set_reg16(rr, value);
    }
}

fn op_ld_sp_hl<B: FastBus>(env: &mut FastEnv<B>) {
    let value = env.reg16or24_ext(Reg16::HL);
    if env.state.is_op_long() {
        env.set_reg24(Reg16::SP, value);
    } else {
        env.set_reg16(Reg16::SP, value as u16);
    }
}

fn op_inc_r<B: FastBus>(env: &mut FastEnv<B>, reg: Reg8) {
    let a = env.reg8_ext(reg);
    let value = op_inc(env, a);
    if reg == Reg8::_HL {
        env.use_cycles(1);
    }
    env.set_reg(reg, value);
}

fn op_dec_r<B: FastBus>(env: &mut FastEnv<B>, reg: Reg8) {
    let a = env.reg8_ext(reg);
    let value = op_dec(env, a);
    if reg == Reg8::_HL {
        env.use_cycles(1);
    }
    env.set_reg(reg, value);
}

fn op_inc_dec_rr<B: FastBus>(env: &mut FastEnv<B>, rr: Reg16, inc: bool) {
    let delta = if inc { 1 } else { -1_i32 as u32 };
    let value = env.reg16or24_ext(rr).wrapping_add(delta);
    if env.state.is_op_long() {
        env.set_reg24(rr, value);
    } else {
        env.set_reg16(rr, value as u16);
    }
}

fn op_add_hl_rr<B: FastBus>(env: &mut FastEnv<B>, rr: Reg16) {
    let aa = env.index_value();
    let bb = env.reg16or24_ext(rr);
    env.state.reg.set_memptr((aa as u16).wrapping_add(1));
    if env.state.is_op_long() {
        let value = op_add24(env, aa, bb);
        env.set_reg24(Reg16::HL, value);
    } else {
        let value = op_add16(env, aa as u16, bb as u16);
        env.set_reg16(Reg16::HL, value);
    }
}

fn op_adc_hl_rr<B: FastBus>(env: &mut FastEnv<B>, rr: Reg16) {
    let aa = env.index_value();
    let bb = env.reg16or24_ext(rr);
    env.state.reg.set_memptr((aa as u16).wrapping_add(1));
    if env.state.is_op_long() {
        let value = op_adc24(env, aa, bb);
        env.state.reg.set24(Reg16::HL, value);
    } else {
        let value = op_adc16(env, aa as u16, bb as u16);
        env.state.reg.set16(Reg16::HL, value);
    }
}

fn op_sbc_hl_rr<B: FastBus>(env: &mut FastEnv<B>, rr: Reg16) {
    let aa = env.index_value();
    let bb = env.reg16or24_ext(rr);
    env.state.reg.set_memptr((aa as u16).wrapping_add(1));
    if env.state.is_op_long() {
        let value = op_sbc24(env, aa, bb);
        env.state.reg.set24(Reg16::HL, value);
    } else {
        let value = op_sbc16(env, aa as u16, bb as u16);
        env.state.reg.set16(Reg16::HL, value);
    }
}

fn op_alu_a_r<B: FastBus>(env: &mut FastEnv<B>, op: usize, reg: Reg8) {
    let a = env.state.reg.a();
    let b = env.reg8_ext(reg);
    let value = op_alu(env, op, a, b);
    env.state.reg.set_a(value);
}

fn op_alu_a_n<B: FastBus>(env: &mut FastEnv<B>, op: usize) {
    let a = env.state.reg.a();
    let b = env.advance_pc();
    let value = op_alu(env, op, a, b);
    env.state.reg.set_a(value);
}

fn op_alu_a_idx_offset<B: FastBus>(env: &mut FastEnv<B>, op: usize, index: Reg16) {
    let offset = env.advance_pc() as i8 as i32 as u32;
    let a = env.state.reg.a();
    let address = if env.state.is_op_long() {
        env.state.reg.get24(index).wrapping_add(offset)
    } else {
        env.state.reg.get16_mbase_offset(index, offset as u16)
    };
    let b = env.read8(address);
    let value = op_alu(env, op, a, b);
    env.state.reg.set_a(value);
}

fn op_neg<B: FastBus>(env: &mut FastEnv<B>) {
    let b = env.state.reg.a();
    let value = op_sub(env, 0, b);
    env.state.reg.set_a(value);
}

fn op_daa<B: FastBus>(env: &mut FastEnv<B>) {
    let a = env.state.reg.a();
    let hi = a >> 4;
    let lo = a & 0x0f;
    let nf = env.state.reg.get_flag(Flag::N);
    let cf = env.state.reg.get_flag(Flag::C);
    let hf = env.state.reg.get_flag(Flag::H);
    let lo6 = hf || lo > 9;
    let hi6 = cf || hi > 9 || (hi == 9 && lo > 9);
    let diff = if lo6 { 6 } else { 0 } + if hi6 { 6 << 4 } else { 0 };
    let new_a = if nf {
        a.wrapping_sub(diff)
    } else {
        a.wrapping_add(diff)
    };
    env.state.reg.set_a(new_a);
    env.state.reg.update_sz53_flags(new_a);
    env.state.reg.update_p_flag(new_a);
    let new_hf = (!nf && lo > 9) || (nf && hf && lo < 6);
    env.state.reg.put_flag(Flag::H, new_hf);
    env.state.reg.put_flag(Flag::C, hi6);
}

fn op_cpl<B: FastBus>(env: &mut FastEnv<B>) {
    let value = !env.state.reg.a();
    env.state.reg.set_a(value);
    env.state.reg.update_hn_flags(true, true);
    env.state.reg.update_undocumented_flags(value);
}

fn op_scf<B: FastBus>(env: &mut FastEnv<B>) {
    let a = env.state.reg.a();
    let old_f = env.state.reg.get8(Reg8::F);
    let flag_source = if env.state.reg.flag_q() { a } else { a | old_f };
    env.state.reg.set_flag(Flag::C);
    env.state.reg.update_hn_flags(false, false);
    env.state.reg.update_undocumented_flags(flag_source);
}

fn op_ccf<B: FastBus>(env: &mut FastEnv<B>) {
    let a = env.state.reg.a();
    let old_f = env.state.reg.get8(Reg8::F);
    let flag_source = if env.state.reg.flag_q() { a } else { a | old_f };
    let c = env.state.reg.get_flag(Flag::C);
    env.state.reg.put_flag(Flag::C, !c);
    env.state.reg.update_hn_flags(c, false);
    env.state.reg.update_undocumented_flags(flag_source);
}

fn op_rot_r<B: FastBus>(
    env: &mut FastEnv<B>,
    reg: Reg8,
    (dir, mode): (ShiftDir, ShiftMode),
    fast: bool,
    indexed: bool,
) {
    let mut value = if indexed {
        env.reg8_ext(Reg8::_HL)
    } else {
        env.reg8_ext(reg)
    };
    let carry;
    match dir {
        ShiftDir::Left => {
            let upper_bit = value >= 0x80;
            value <<= 1;
            let set_lower_bit = match mode {
                ShiftMode::Arithmetic => false,
                ShiftMode::Logical => true,
                ShiftMode::Rotate => env.state.reg.get_flag(Flag::C),
                ShiftMode::RotateCarry => upper_bit,
            };
            if set_lower_bit {
                value |= 1;
            }
            carry = upper_bit;
        }
        ShiftDir::Right => {
            let upper_bit = value >= 0x80;
            let lower_bit = (value & 1) == 1;
            value >>= 1;
            let set_upper_bit = match mode {
                ShiftMode::Arithmetic => upper_bit,
                ShiftMode::Logical => false,
                ShiftMode::Rotate => env.state.reg.get_flag(Flag::C),
                ShiftMode::RotateCarry => lower_bit,
            };
            if set_upper_bit {
                value |= 0x80;
            }
            carry = lower_bit;
        }
    }
    if indexed && reg != Reg8::_HL {
        env.set_reg(Reg8::_HL, value);
    }
    if indexed && reg != Reg8::_HL {
        env.state.reg.set8(reg, value);
    } else {
        env.set_reg(reg, value);
    }
    if reg == Reg8::_HL {
        env.use_cycles(1);
    }
    env.state.reg.put_flag(Flag::C, carry);
    env.state.reg.update_hn_flags(false, false);
    if fast {
        env.state.reg.update_undocumented_flags(value);
    } else {
        env.state.reg.update_bits_in_flags(value);
    }
}

fn op_bit_r<B: FastBus>(env: &mut FastEnv<B>, bit: u8, reg: Reg8) {
    let value = env.reg8_ext(reg);
    let z = value & (1 << bit);
    env.state.reg.put_flag(Flag::S, (z & 0x80) != 0);
    env.state.reg.put_flag(Flag::Z, z == 0);
    env.state.reg.put_flag(Flag::P, z == 0);
    env.state.reg.set_flag(Flag::H);
    env.state.reg.clear_flag(Flag::N);
    if reg == Reg8::_HL {
        env.use_cycles(1);
        let flag_source = if env.is_alt_index() {
            (env.index_address() >> 8) as u8
        } else {
            (env.state.reg.memptr >> 8) as u8
        };
        env.state.reg.update_undocumented_flags(flag_source);
    } else {
        env.state.reg.update_undocumented_flags(value);
    }
}

fn op_set_res_r<B: FastBus>(env: &mut FastEnv<B>, bit: u8, reg: Reg8, set: bool) {
    let mut value = env.reg8_ext(reg);
    if set {
        value |= 1 << bit;
    } else {
        value &= !(1 << bit);
    }
    if reg == Reg8::_HL {
        env.use_cycles(1);
    }
    env.set_reg(reg, value);
}

fn op_indexed_set_res_r<B: FastBus>(env: &mut FastEnv<B>, bit: u8, reg: Reg8, set: bool) {
    let mut value = env.reg8_ext(Reg8::_HL);
    if set {
        value |= 1 << bit;
    } else {
        value &= !(1 << bit);
    }
    env.set_reg(Reg8::_HL, value);
    if reg != Reg8::_HL {
        env.state.reg.set8(reg, value);
    }
}

fn op_rxd<B: FastBus>(env: &mut FastEnv<B>, dir: ShiftDir) {
    let mut a = env.state.reg.a();
    let mut phl = env.reg8_ext(Reg8::_HL);
    match dir {
        ShiftDir::Left => {
            let temp = (a & 0xf0) | (phl >> 4);
            phl = (phl << 4) | (a & 0x0f);
            a = temp;
        }
        ShiftDir::Right => {
            let temp = (a & 0xf0) | (phl & 0x0f);
            phl = (a << 4) | (phl >> 4);
            a = temp;
        }
    }
    env.state.reg.set_a(a);
    env.set_reg(Reg8::_HL, phl);
    env.state.reg.update_bits_in_flags(a);
}

fn relative_jump<B: FastBus>(env: &mut FastEnv<B>, offset: u8) {
    let pc = env.wrap_address(env.state.pc(), offset as i8 as i32);
    env.state.set_pc(pc);
}

fn handle_jump_adl_state<B: FastBus>(env: &mut FastEnv<B>) -> bool {
    if env.state.reg.adl {
        match env.state.sz_prefix {
            SizePrefix::SIS => env.state.reg.adl = false,
            SizePrefix::LIS | SizePrefix::SIL => {
                env.trap_illegal_instruction();
                return false;
            }
            SizePrefix::LIL | SizePrefix::None => {}
        }
    } else {
        match env.state.sz_prefix {
            SizePrefix::LIL => env.state.reg.adl = true,
            SizePrefix::LIS | SizePrefix::SIL => {
                env.trap_illegal_instruction();
                return false;
            }
            SizePrefix::SIS | SizePrefix::None => {}
        }
    }
    true
}

fn op_djnz<B: FastBus>(env: &mut FastEnv<B>) {
    let offset = env.advance_pc();
    let b = env.state.reg.get8(Reg8::B).wrapping_add(0xff);
    env.state.reg.set8(Reg8::B, b);
    if b != 0 {
        env.use_cycles(2);
        relative_jump(env, offset);
    }
}

fn op_jr<B: FastBus>(env: &mut FastEnv<B>) {
    let offset = env.advance_pc();
    env.use_cycles(1);
    relative_jump(env, offset);
}

fn op_jr_cc<B: FastBus>(env: &mut FastEnv<B>, (flag, value): (Flag, bool)) {
    let offset = env.advance_pc();
    if env.state.reg.get_flag(flag) == value {
        env.use_cycles(2);
        relative_jump(env, offset);
    }
}

fn op_jp<B: FastBus>(env: &mut FastEnv<B>) {
    let address = env.advance_immediate_16mbase_or_24();
    env.use_cycles(1);
    if handle_jump_adl_state(env) {
        env.state.set_pc(address);
    }
}

fn op_jp_cc<B: FastBus>(env: &mut FastEnv<B>, (flag, value): (Flag, bool)) {
    let address = env.advance_immediate_16mbase_or_24();
    if env.state.reg.get_flag(flag) == value {
        env.use_cycles(1);
        env.state.set_pc(address);
    }
}

fn op_jp_hl<B: FastBus>(env: &mut FastEnv<B>) {
    let address = env.index_value();
    env.use_cycles(1);
    if handle_jump_adl_state(env) {
        env.state.set_pc(address);
    }
}

fn handle_call_size_prefix<B: FastBus>(env: &mut FastEnv<B>) {
    let pc = env.state.pc();
    if env.state.reg.adl {
        match env.state.sz_prefix {
            SizePrefix::None => env.push(pc),
            SizePrefix::SIS | SizePrefix::LIS => {
                env.push_byte_sps((pc >> 8) as u8);
                env.push_byte_sps(pc as u8);
                env.push_byte_spl((pc >> 16) as u8);
                env.push_byte_spl(3);
                env.state.reg.adl = false;
            }
            SizePrefix::LIL => {
                env.push(pc);
                env.push_byte_spl(3);
            }
            SizePrefix::SIL => env.push(pc),
        }
    } else {
        match env.state.sz_prefix {
            SizePrefix::None => {
                env.push_byte_sps((pc >> 8) as u8);
                env.push_byte_sps(pc as u8);
            }
            SizePrefix::LIL | SizePrefix::SIL => {
                env.push_byte_spl((pc >> 8) as u8);
                env.push_byte_spl(pc as u8);
                env.push_byte_spl(2);
                env.state.reg.adl = true;
            }
            SizePrefix::SIS => {
                env.push_byte_sps((pc >> 8) as u8);
                env.push_byte_sps(pc as u8);
                env.push_byte_spl(2);
            }
            SizePrefix::LIS => {
                env.push_byte_spl((pc >> 8) as u8);
                env.push_byte_spl(pc as u8);
            }
        }
    }
}

fn op_call<B: FastBus>(env: &mut FastEnv<B>) {
    let address = env.advance_immediate16or24();
    handle_call_size_prefix(env);
    env.state.set_pc(address);
}

fn op_call_cc<B: FastBus>(env: &mut FastEnv<B>, (flag, value): (Flag, bool)) {
    let address = env.advance_immediate_16mbase_or_24();
    if env.state.reg.get_flag(flag) == value {
        handle_call_size_prefix(env);
        env.state.set_pc(address);
    }
}

fn handle_rst_size_prefix<B: FastBus>(env: &mut FastEnv<B>, vec: u32) {
    let pc = env.state.pc();
    if env.state.reg.adl {
        match env.state.sz_prefix {
            SizePrefix::None => {
                env.push(pc);
                env.state.set_pc(vec);
            }
            SizePrefix::SIL | SizePrefix::SIS => {
                env.push_byte_sps((pc >> 8) as u8);
                env.push_byte_sps(pc as u8);
                env.push_byte_spl((pc >> 16) as u8);
                env.push_byte_spl(3);
                env.state.reg.pc = vec;
                env.state.reg.adl = false;
            }
            SizePrefix::LIS | SizePrefix::LIL => {
                env.push(pc);
                env.push_byte_spl(3);
                env.state.reg.pc = vec;
            }
        }
    } else {
        match env.state.sz_prefix {
            SizePrefix::None => {
                env.push(pc);
                env.state.set_pc(vec);
            }
            SizePrefix::SIL | SizePrefix::SIS => {
                env.push(pc);
                env.push_byte_spl(2);
                env.state.reg.pc = vec;
            }
            SizePrefix::LIL | SizePrefix::LIS => {
                env.push_byte_spl((pc >> 8) as u8);
                env.push_byte_spl(pc as u8);
                env.push_byte_spl(2);
                env.state.reg.adl = true;
                env.state.reg.pc = vec;
            }
        }
    }
}

fn op_rst<B: FastBus>(env: &mut FastEnv<B>, address: u8) {
    handle_rst_size_prefix(env, address as u32);
}

fn op_ret<B: FastBus>(env: &mut FastEnv<B>) {
    env.use_cycles(2);
    env.subroutine_return();
}

fn op_reti<B: FastBus>(env: &mut FastEnv<B>) {
    env.use_cycles(2);
    env.subroutine_return();
}

fn op_retn<B: FastBus>(env: &mut FastEnv<B>) {
    env.use_cycles(2);
    env.subroutine_return();
    env.state.reg.end_nmi();
}

fn op_ret_cc<B: FastBus>(env: &mut FastEnv<B>, (flag, value): (Flag, bool)) {
    if env.state.reg.get_flag(flag) == value {
        env.use_cycles(2);
        env.subroutine_return();
    } else {
        env.use_cycles(1);
    }
}

fn op_push_rr<B: FastBus>(env: &mut FastEnv<B>, rr: Reg16) {
    let value = env.reg16or24_ext(rr);
    env.push(value);
}

fn op_pop_rr<B: FastBus>(env: &mut FastEnv<B>, rr: Reg16) {
    let value = env.pop();
    if env.state.is_op_long() && rr != Reg16::AF {
        env.set_reg24(rr, value);
    } else {
        env.set_reg16(rr, value as u16);
    }
}

fn op_exx<B: FastBus>(env: &mut FastEnv<B>) {
    env.state.reg.swap24(Reg16::BC);
    env.state.reg.swap24(Reg16::DE);
    env.state.reg.swap24(Reg16::HL);
}

fn op_ex_de_hl<B: FastBus>(env: &mut FastEnv<B>) {
    if env.state.is_op_long() {
        let temp = env.state.reg.get24(Reg16::HL);
        env.state
            .reg
            .set24(Reg16::HL, env.state.reg.get24(Reg16::DE));
        env.state.reg.set24(Reg16::DE, temp);
    } else {
        let temp = env.state.reg.get16(Reg16::HL);
        env.state
            .reg
            .set16(Reg16::HL, env.state.reg.get16(Reg16::DE));
        env.state.reg.set16(Reg16::DE, temp);
    }
}

fn op_ex_psp_hl<B: FastBus>(env: &mut FastEnv<B>) {
    let address = env.state.sp();
    let temp = env.reg16or24_ext(Reg16::HL);
    if env.state.is_op_long() {
        let value = env.read24(address);
        env.set_reg24(Reg16::HL, value);
        env.write24(address, temp);
    } else {
        let value = env.read16(address);
        env.set_reg16_preserve_17_to_24(Reg16::HL, value);
        env.write16(address, temp as u16);
    }
}

fn op_lea_rr_ind_offset<B: FastBus>(env: &mut FastEnv<B>, dest: Reg16, src: Reg16) {
    let imm = env.advance_pc() as i8 as i32 as u32;
    if env.state.is_op_long() {
        let value = env.state.reg.get24(src).wrapping_add(imm);
        env.state.reg.set24(dest, value);
    } else {
        let value = env.state.reg.get16(src).wrapping_add(imm as u16);
        env.state.reg.set16(dest, value);
    }
}

fn op_pea<B: FastBus>(env: &mut FastEnv<B>, src: Reg16) {
    let imm = env.advance_pc() as i8 as i32 as u32;
    if env.state.is_op_long() {
        let value = env.state.reg.get24(src).wrapping_add(imm);
        env.push(value);
    } else {
        let value = env.state.reg.get16(src).wrapping_add(imm as u16);
        env.push(value as u32);
    }
}

fn op_tst_a_r<B: FastBus>(env: &mut FastEnv<B>, reg: Reg8) {
    let a = env.state.reg.a();
    let b = env.reg8_ext(reg);
    op_tst(env, a, b);
}

fn op_tst_a_n<B: FastBus>(env: &mut FastEnv<B>) {
    let a = env.state.reg.a();
    let b = env.advance_pc();
    op_tst(env, a, b);
}

fn op_mlt_rr<B: FastBus>(env: &mut FastEnv<B>, reg: Reg16) {
    let r = env.state.reg.get16(reg);
    let a = r & 0xff;
    let b = (r >> 8) & 0xff;
    env.state.reg.set16(reg, a * b);
    env.use_cycles(4);
}

fn op_out_c_r<B: FastBus>(env: &mut FastEnv<B>, reg: Reg8) {
    let address = env.state.reg.get16(Reg16::BC);
    let value = env.state.reg.get8(reg);
    env.output8(address, value);
}

fn op_out_c_0<B: FastBus>(env: &mut FastEnv<B>) {
    let address = env.state.reg.get16(Reg16::BC);
    env.output8(address, 0);
}

fn op_out_n_a<B: FastBus>(env: &mut FastEnv<B>) {
    let a = env.state.reg.a();
    let address = ((a as u16) << 8) + env.advance_pc() as u16;
    env.output8(address, a);
}

fn op_out0_n_r<B: FastBus>(env: &mut FastEnv<B>, reg: Reg8) {
    let address = env.advance_pc() as u16;
    let value = env.state.reg.get8(reg);
    env.output8(address, value);
}

fn op_in0_r_n<B: FastBus>(env: &mut FastEnv<B>, reg: Reg8) {
    let address = env.advance_pc() as u16;
    let value = env.input8(address);
    env.state
        .reg
        .update_arithmetic_flags(value as u16, value as u16, value as u16, true, false);
    env.state.reg.set8(reg, value);
}

fn op_in_r_c<B: FastBus>(env: &mut FastEnv<B>, reg: Reg8) {
    let address = env.state.reg.get16(Reg16::BC);
    let value = env.input8(address);
    env.state.reg.set8(reg, value);
    env.state.reg.update_bits_in_flags(value);
}

fn op_in_0_c<B: FastBus>(env: &mut FastEnv<B>) {
    let address = env.state.reg.get16(Reg16::BC);
    let value = env.input8(address);
    env.state.reg.update_bits_in_flags(value);
}

fn op_in_a_n<B: FastBus>(env: &mut FastEnv<B>) {
    let a = env.state.reg.a();
    let address = ((a as u16) << 8) + env.advance_pc() as u16;
    let value = env.input8(address);
    env.state.reg.set_a(value);
}

fn op_tstio_n<B: FastBus>(env: &mut FastEnv<B>) {
    let value = env.input8(env.state.reg.get8(Reg8::C) as u16);
    let mask = env.advance_pc();
    op_tst(env, value, mask);
}

fn set_simple_block_io_flags<B: FastBus>(env: &mut FastEnv<B>, counter: u32, value: u8) {
    env.state.reg.put_flag(Flag::Z, counter == 0);
    env.state.reg.put_flag(Flag::N, value & 0x80 != 0);
}

fn repeat_ed_instruction<B: FastBus>(env: &mut FastEnv<B>, counter: u32) {
    if counter != 0 {
        let instruction_len = match env.state.sz_prefix {
            SizePrefix::None => 2,
            _ => 3,
        };
        let pc = env.wrap_address(env.state.pc(), -instruction_len);
        env.state.set_pc(pc);
    }
}

fn op_in_block2<B: FastBus>(env: &mut FastEnv<B>, inc: bool, repeat: bool) {
    let address = if repeat {
        env.state.reg.get16(Reg16::DE)
    } else {
        env.state.reg.get16(Reg16::BC)
    };
    let value = env.input8(address);
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
}

fn op_out_block2<B: FastBus>(env: &mut FastEnv<B>, inc: bool, repeat: bool) {
    let value = env.reg8_ext(Reg8::_HL);
    let address = if repeat {
        env.state.reg.get16(Reg16::DE)
    } else {
        env.state.reg.get16(Reg16::BC)
    };
    env.output8(address, value);
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
}

fn op_otim_otdm<B: FastBus>(env: &mut FastEnv<B>, inc: bool, repeat: bool) {
    let value = env.reg8_ext(Reg8::_HL);
    let address = env.state.reg.get8(Reg8::C) as u16;
    env.output8(address, value);
    let b = env.state.reg.inc_dec8(Reg8::B, false) as u32;
    env.state.reg.inc_dec8(Reg8::C, inc);
    inc_dec16or24(env, Reg16::HL, inc);
    set_simple_block_io_flags(env, b, value);
    if repeat {
        repeat_ed_instruction(env, b);
    }
}

fn op_inirx_or_indrx<B: FastBus>(env: &mut FastEnv<B>, inc: bool) {
    let address = env.state.reg.get16(Reg16::DE);
    let value = env.input8(address);
    env.set_reg(Reg8::_HL, value);
    let counter = inc_dec16or24(env, Reg16::BC, false);
    inc_dec16or24(env, Reg16::HL, inc);
    set_simple_block_io_flags(env, counter, value);
    repeat_ed_instruction(env, counter);
}

fn op_in_block<B: FastBus>(env: &mut FastEnv<B>, (inc, repeat): (bool, bool)) {
    let original_bc = env.state.reg.get16(Reg16::BC);
    let b = env.state.reg.inc_dec8(Reg8::B, false);
    let address = env.state.reg.get16(Reg16::BC);
    let value = env.input8(address);
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
        let pc = env.wrap_address(env.state.pc(), -2);
        env.state.set_pc(pc);
    }
}

fn op_out_block<B: FastBus>(env: &mut FastEnv<B>, (inc, repeat): (bool, bool)) {
    let address = env.state.reg.get16(Reg16::BC);
    let b = env.state.reg.inc_dec8(Reg8::B, false);
    let value = env.reg8_ext(Reg8::_HL);
    env.output8(address, value);
    inc_dec16or24(env, Reg16::HL, inc);
    let k = value as u16 + env.state.reg.get8(Reg8::L) as u16;
    env.state.reg.update_block_flags(value, k, b);
    if repeat && b != 0 {
        let pc = env.wrap_address(env.state.pc(), -2);
        env.state.set_pc(pc);
    }
}

fn op_otirx_or_otdrx<B: FastBus>(env: &mut FastEnv<B>, inc: bool) {
    let value = env.reg8_ext(Reg8::_HL);
    let address = env.state.reg.get16(Reg16::DE);
    env.use_cycles(1);
    let bc = if env.state.is_op_long() {
        env.state.reg.inc_dec24(Reg16::HL, inc);
        env.state.reg.inc_dec24(Reg16::BC, false)
    } else {
        env.state.reg.inc_dec16(Reg16::HL, inc);
        env.state.reg.inc_dec16(Reg16::BC, false)
    };
    if env.state.cached_instruction {
        env.use_cycles(-2);
    }
    env.output8(address, value);
    env.state.reg.put_flag(Flag::Z, bc == 0);
    env.state.reg.put_flag(Flag::N, value & 0x80 == 0x80);
    if bc != 0 {
        let instruction_len = match env.state.sz_prefix {
            SizePrefix::None => 2,
            _ => 3,
        };
        let pc = env.wrap_address(env.state.pc(), -instruction_len);
        env.state.set_pc(pc);
        if !matches!(env.state.sz_prefix, SizePrefix::None) {
            env.use_cycles(-1);
        }
        env.state.cached_instruction = true;
    } else {
        env.state.cached_instruction = false;
    }
}

fn op_ld_block<B: FastBus>(env: &mut FastEnv<B>, (inc, repeat): (bool, bool)) {
    let value = env.reg8_ext(Reg8::_HL);
    let address = env.reg16mbase_or_24(Reg16::DE);
    env.write8(address, value);
    env.use_cycles(1);
    let bc = if env.state.is_op_long() {
        env.state.reg.inc_dec24(Reg16::DE, inc);
        env.state.reg.inc_dec24(Reg16::HL, inc);
        env.state.reg.inc_dec24(Reg16::BC, false)
    } else {
        env.state.reg.inc_dec16(Reg16::DE, inc);
        env.state.reg.inc_dec16(Reg16::HL, inc);
        env.state.reg.inc_dec16(Reg16::BC, false)
    };
    let n = value.wrapping_add(env.state.reg.a());
    env.state.reg.update_undocumented_flags_block(n);
    env.state.reg.clear_flag(Flag::N);
    env.state.reg.clear_flag(Flag::H);
    env.state.reg.put_flag(Flag::P, bc != 0);
    if repeat && bc != 0 {
        let instruction_len = match env.state.sz_prefix {
            SizePrefix::None => 2,
            _ => 3,
        };
        let pc = env.wrap_address(env.state.pc(), -instruction_len);
        env.state.set_pc(pc);
        env.use_cycles(-2);
        if !matches!(env.state.sz_prefix, SizePrefix::None) {
            env.use_cycles(-1);
        }
    }
}

fn op_cp_block<B: FastBus>(env: &mut FastEnv<B>, (inc, repeat): (bool, bool)) {
    let a = env.state.reg.a();
    let b = env.reg8_ext(Reg8::_HL);
    let c_bak = env.state.reg.get_flag(Flag::C);
    op_cp(env, a, b);
    let bc = if env.state.is_op_long() {
        env.state.reg.inc_dec24(Reg16::HL, inc);
        env.state.reg.inc_dec24(Reg16::BC, false)
    } else {
        env.state.reg.inc_dec16(Reg16::HL, inc);
        env.state.reg.inc_dec16(Reg16::BC, false)
    };
    let mut n = a.wrapping_sub(b);
    if env.state.reg.get_flag(Flag::H) {
        n = n.wrapping_sub(1);
    }
    env.state.reg.update_undocumented_flags_block(n);
    env.state.reg.set_flag(Flag::N);
    env.state.reg.put_flag(Flag::P, bc != 0);
    env.state.reg.put_flag(Flag::C, c_bak);
    if repeat && bc != 0 && a != b {
        let pc = env.wrap_address(env.state.pc(), -2);
        env.state.set_pc(pc);
    }
}

fn op_ld_idx_disp_rr<B: FastBus>(env: &mut FastEnv<B>, index: Reg16, src: Reg16) {
    let imm = env.advance_pc() as i8 as i32 as u32;
    if env.state.is_op_long() {
        let value = env.state.reg.get24(src);
        let address = env.state.reg.get24(index).wrapping_add(imm);
        env.write24(address, value);
    } else {
        let value = env.state.reg.get16(src);
        let address = env.state.reg.get16_mbase(index).wrapping_add(imm);
        env.write16(address, value);
    }
}

fn op_ld_rr_idx_disp<B: FastBus>(env: &mut FastEnv<B>, dest: Reg16, index: Reg16) {
    let imm = env.advance_pc() as i8 as i32 as u32;
    if env.state.is_op_long() {
        let address = env.state.reg.get24(index).wrapping_add(imm);
        let value = env.read24(address);
        env.state.reg.set24(dest, value);
    } else {
        let address = env.state.reg.get16_mbase(index).wrapping_add(imm);
        let value = env.read16(address);
        env.state.reg.set16(dest, value);
    }
}

fn op_ld_rr_ind_hl<B: FastBus>(env: &mut FastEnv<B>, dest: Reg16) {
    if env.state.is_op_long() {
        let address = env.state.reg.get24(Reg16::HL);
        let value = env.read24(address);
        env.state.reg.set24(dest, value);
    } else {
        let address = env.state.reg.get16_mbase(Reg16::HL);
        let value = env.read16(address);
        env.state.reg.set16(dest, value);
    }
}

fn op_ld_ind_hl_rr<B: FastBus>(env: &mut FastEnv<B>, src: Reg16) {
    if env.state.is_op_long() {
        let address = env.state.reg.get24(Reg16::HL);
        let value = env.state.reg.get24(src);
        env.write24(address, value);
    } else {
        let address = env.state.reg.get16_mbase(Reg16::HL);
        let value = env.state.reg.get16(src);
        env.write16(address, value);
    }
}
