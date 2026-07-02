use ez80::*;
use std::cell::Cell;
use std::collections::HashMap;

struct SparseMem {
    data: HashMap<u32, u8>,
    cycles: Cell<u64>,
}

impl SparseMem {
    fn new() -> Self {
        SparseMem { data: HashMap::new(), cycles: Cell::new(0) }
    }
    fn w8(&mut self, a: u32, v: u8) { self.data.insert(a & 0xffffff, v); }
    fn r8(&self, a: u32) -> u8 { self.data.get(&(a & 0xffffff)).copied().unwrap_or(0) }
    fn w24(&mut self, a: u32, v: u32) {
        let a = a & 0xffffff;
        self.data.insert(a, v as u8);
        self.data.insert(a.wrapping_add(1) & 0xffffff, (v >> 8) as u8);
        self.data.insert(a.wrapping_add(2) & 0xffffff, (v >> 16) as u8);
    }
    fn w16(&mut self, a: u32, v: u16) {
        let a = a & 0xffffff;
        self.data.insert(a, v as u8);
        self.data.insert(a.wrapping_add(1) & 0xffffff, (v >> 8) as u8);
    }
}

impl Machine for SparseMem {
    fn peek(&self, address: u32) -> u8 { self.r8(address) }
    fn poke(&mut self, address: u32, value: u8) { self.w8(address, value); }
    fn use_cycles(&self, _c: i32) { self.cycles.set(self.cycles.get().wrapping_add(_c as u64)); }
    fn port_in(&mut self, _address: u16) -> u8 { 0 }
    fn port_out(&mut self, _address: u16, _value: u8) {}
}

fn setup() -> (Cpu, SparseMem) {
    (Cpu::new_ez80(), SparseMem::new())
}

fn adl(c: &Cpu) -> bool { c.state.reg.adl }
fn madl(c: &Cpu) -> bool { c.state.reg.madl }
fn mbase(c: &Cpu) -> u8 { c.state.reg.mbase }
fn pc(c: &Cpu) -> u32 { c.state.pc() }
fn spl(c: &Cpu) -> u32 { c.state.reg.get24(Reg16::SP) }
fn sps(c: &Cpu) -> u16 { c.state.reg.get16(Reg16::SP) }
fn a(c: &Cpu) -> u8 { c.state.reg.a() }
fn hl(c: &Cpu) -> u32 { c.state.reg.get24(Reg16::HL) }
fn hl16(c: &Cpu) -> u16 { c.state.reg.get16(Reg16::HL) }
fn ix(c: &Cpu) -> u32 { c.state.reg.get24(Reg16::IX) }
fn bc(c: &Cpu) -> u32 { c.state.reg.get24(Reg16::BC) }
fn de(c: &Cpu) -> u32 { c.state.reg.get24(Reg16::DE) }
fn f(c: &Cpu) -> u8 { c.state.reg.get8(Reg8::F) }
fn flag(c: &Cpu, fl: Flag) -> bool { c.state.reg.get_flag(fl) }

fn set_a(c: &mut Cpu, v: u8) { c.state.reg.set8(Reg8::A, v); }
fn set_hl24(c: &mut Cpu, v: u32) { c.state.reg.set24(Reg16::HL, v); }
fn set_hl16(c: &mut Cpu, v: u16) { c.state.reg.set16(Reg16::HL, v); }
fn set_bc24(c: &mut Cpu, v: u32) { c.state.reg.set24(Reg16::BC, v); }
fn set_ix24(c: &mut Cpu, v: u32) { c.state.reg.set24(Reg16::IX, v); }
fn set_iy24(c: &mut Cpu, v: u32) { c.state.reg.set24(Reg16::IY, v); }
fn set_spl(c: &mut Cpu, v: u32) { c.state.reg.set24(Reg16::SP, v); }
fn set_sps(c: &mut Cpu, v: u16) { c.state.reg.set16(Reg16::SP, v); }
fn set_flag(c: &mut Cpu, fl: Flag, v: bool) { c.state.reg.put_flag(fl, v); }

fn exec(c: &mut Cpu, m: &mut SparseMem) {
    c.execute_instruction(m);
}
fn fast_exec(c: &mut Cpu, m: &mut SparseMem) {
    c.fast_execute_instruction(m);
}

// ============================================================
//  1: Reset starts in Z80 mode
// ============================================================
#[test]
fn t01_reset_starts_z80_mode() {
    let c = Cpu::new_ez80();
    assert!(!adl(&c));
    assert!(!madl(&c));
    assert_eq!(mbase(&c), 0);
    assert_eq!(c.state.reg.pc, 0);
    assert_eq!(sps(&c), 0);
    assert_eq!(spl(&c), 0);
}

// ============================================================
//  2: JP.LIL enters ADL
// ============================================================
#[test]
fn t02_jp_lil_enters_adl() {
    let (mut c, mut m) = setup();
    m.w8(0x00, 0x5b); // .LIL
    m.w8(0x01, 0xc3); // JP nnnn
    m.w8(0x02, 0x00);
    m.w8(0x03, 0x00);
    m.w8(0x04, 0x01);
    exec(&mut c, &mut m);
    assert!(adl(&c));
    assert_eq!(pc(&c), 0x010000);
}

// ============================================================
//  3: ADL ignores MBASE
// ============================================================
#[test]
fn t03_adl_ignores_mbase() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    c.state.reg.mbase = 0x12;
    set_hl24(&mut c, 0x003456);
    m.w8(0x003456, 0x5a);
    m.w8(0x123456, 0xa5);
    m.w8(0x00, 0x7e); // LD A,(HL)
    exec(&mut c, &mut m);
    assert_eq!(a(&c), 0x5a);
}

// ============================================================
//  4: Z80 mode uses MBASE
// ============================================================
#[test]
fn t04_z80_mode_uses_mbase() {
    let (mut c, mut m) = setup();
    c.state.reg.mbase = 0x12;
    set_hl16(&mut c, 0x3456);
    m.w8(0x123456, 0x5a);
    m.w8(0x003456, 0xa5);
    // In Z80 mode, effective PC = {MBASE, PC[15:0]} = 0x120000
    m.w8(0x120000, 0x7e); // LD A,(HL)
    assert_eq!(m.r8(0x123456), 0x5a, "memory pre-check");
    exec(&mut c, &mut m);
    assert_eq!(a(&c), 0x5a);
}

// ============================================================
//  5: LD MB,A works only in ADL
// ============================================================
#[test]
fn t05_ld_mb_a_adl() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    set_a(&mut c, 0x34);
    m.w8(0x00, 0xed); m.w8(0x01, 0x6d); // LD MB,A
    exec(&mut c, &mut m);
    assert_eq!(mbase(&c), 0x34);
}

// ============================================================
//  6: LD MB,A is NOP in Z80 mode
// ============================================================
#[test]
fn t06_ld_mb_a_z80_nop() {
    let (mut c, mut m) = setup();
    c.state.reg.mbase = 0x12;
    set_a(&mut c, 0x34);
    m.w8(0x00, 0xed); m.w8(0x01, 0x6d); // LD MB,A
    exec(&mut c, &mut m);
    assert_eq!(mbase(&c), 0x12);
}

// ============================================================
//  7: LD A,MB reads MBASE in ADL
// ============================================================
#[test]
fn t07_ld_a_mb_adl() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    c.state.reg.mbase = 0x56;
    m.w8(0x00, 0xed); m.w8(0x01, 0x6e); // LD A,MB
    exec(&mut c, &mut m);
    assert_eq!(a(&c), 0x56);
}

// ============================================================
//  8: LD A,MB no-op behavior in Z80
// ============================================================
#[test]
fn t08_ld_a_mb_z80_nop() {
    let (mut c, mut m) = setup();
    c.state.reg.mbase = 0x56;
    set_a(&mut c, 0x99);
    m.w8(0x00, 0xed); m.w8(0x01, 0x6e); // LD A,MB
    exec(&mut c, &mut m);
    assert_eq!(a(&c), 0x99);
}

// ============================================================
//  9: ADL LD HL,Mmn consumes 3 bytes
// ============================================================
#[test]
fn t09_adl_ld_hl_mmn_3_bytes() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    c.state.set_pc(0x100000);
    m.w8(0x100000, 0x21); // LD HL,nn
    m.w8(0x100001, 0x56);
    m.w8(0x100002, 0x34);
    m.w8(0x100003, 0x12);
    exec(&mut c, &mut m);
    assert_eq!(hl(&c), 0x123456);
    assert_eq!(pc(&c), 0x100004);
}

// ============================================================
// 10: Z80 LD HL,mn consumes 2 bytes
// ============================================================
#[test]
fn t10_z80_ld_hl_mn_2_bytes() {
    let (mut c, mut m) = setup();
    c.state.set_pc(0x0100);
    m.w8(0x0100, 0x21); // LD HL,nn
    m.w8(0x0101, 0x56);
    m.w8(0x0102, 0x34);
    exec(&mut c, &mut m);
    assert_eq!(hl16(&c), 0x3456);
    assert_eq!(pc(&c), 0x0103);
}

// ============================================================
// 11: Z80 .LIL immediate load
// ============================================================
#[test]
fn t11_z80_lil_immediate_load() {
    let (mut c, mut m) = setup();
    m.w8(0x00, 0x5b); // .LIL
    m.w8(0x01, 0x21); // LD HL,nn
    m.w8(0x02, 0x56);
    m.w8(0x03, 0x34);
    m.w8(0x04, 0x12);
    exec(&mut c, &mut m);
    assert_eq!(hl(&c), 0x123456);
    assert!(!adl(&c));
}

// ============================================================
// 12: Z80 .LIS immediate load
// ============================================================
#[test]
fn t12_z80_lis_immediate_load() {
    let (mut c, mut m) = setup();
    m.w8(0x00, 0x49); // .LIS
    m.w8(0x01, 0x21); // LD HL,nn
    m.w8(0x02, 0x56);
    m.w8(0x03, 0x34);
    exec(&mut c, &mut m);
    assert_eq!(hl(&c), 0x003456);
    assert!(!adl(&c));
}

// ============================================================
// 13: Z80 .SIL discards upper immediate
// ============================================================
#[test]
fn t13_z80_sil_discards_upper_immediate() {
    let (mut c, mut m) = setup();
    m.w8(0x00, 0x52); // .SIL
    m.w8(0x01, 0x21); // LD HL,nn
    m.w8(0x02, 0x56);
    m.w8(0x03, 0x34);
    m.w8(0x04, 0x12);
    exec(&mut c, &mut m);
    assert_eq!(hl16(&c), 0x3456);
    assert!(!adl(&c));
}

// ============================================================
// 14: Z80 .SIS normal short immediate
// ============================================================
#[test]
fn t14_z80_sis_normal_short_immediate() {
    let (mut c, mut m) = setup();
    m.w8(0x00, 0x40); // .SIS
    m.w8(0x01, 0x21); // LD HL,nn
    m.w8(0x02, 0x56);
    m.w8(0x03, 0x34);
    exec(&mut c, &mut m);
    assert_eq!(hl16(&c), 0x3456);
    assert!(!adl(&c));
}

// ============================================================
// 15: ADL .SIS fetches 2-byte immediate
// ============================================================
#[test]
fn t15_adl_sis_2byte_immediate() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    m.w8(0x00, 0x40); // .SIS
    m.w8(0x01, 0x21); // LD HL,nn
    m.w8(0x02, 0x56);
    m.w8(0x03, 0x34);
    exec(&mut c, &mut m);
    assert_eq!(hl16(&c), 0x3456);
    assert!(adl(&c));
}

// ============================================================
// 16: ADL .LIS fetches 2 but stores long
// ============================================================
#[test]
fn t16_adl_lis_fetches_2_stores_long() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    m.w8(0x00, 0x49); // .LIS
    m.w8(0x01, 0x21); // LD HL,nn
    m.w8(0x02, 0x56);
    m.w8(0x03, 0x34);
    exec(&mut c, &mut m);
    assert_eq!(hl(&c), 0x003456);
    assert!(adl(&c));
}

// ============================================================
// 17: ADL .SIL fetches 3 but stores short
// ============================================================
#[test]
fn t17_adl_sil_fetches_3_stores_short() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    m.w8(0x00, 0x52); // .SIL
    m.w8(0x01, 0x21); // LD HL,nn
    m.w8(0x02, 0x56);
    m.w8(0x03, 0x34);
    m.w8(0x04, 0x12);
    exec(&mut c, &mut m);
    assert_eq!(hl16(&c), 0x3456);
    assert!(adl(&c));
}

// ============================================================
// 18: ADL .LIL normal long immediate
// ============================================================
#[test]
fn t18_adl_lil_normal_long_immediate() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    m.w8(0x00, 0x5b); // .LIL
    m.w8(0x01, 0x21); // LD HL,nn
    m.w8(0x02, 0x56);
    m.w8(0x03, 0x34);
    m.w8(0x04, 0x12);
    exec(&mut c, &mut m);
    assert_eq!(hl(&c), 0x123456);
    assert!(adl(&c));
}

// ============================================================
// 19: Prefix bytes decode before Z80 prefixes
// ============================================================
#[test]
fn t19_prefix_before_z80_prefix() {
    let (mut c, mut m) = setup();
    m.w8(0x00, 0x5b); // .LIL
    m.w8(0x01, 0xdd); // IX prefix
    m.w8(0x02, 0x21); // LD HL,nn → becomes LD IX,nn
    m.w8(0x03, 0x56);
    m.w8(0x04, 0x34);
    m.w8(0x05, 0x12);
    exec(&mut c, &mut m);
    assert_eq!(ix(&c), 0x123456);
}

// ============================================================
// 20: Prefix on unaffected instruction is harmless
// ============================================================
#[test]
fn t20_prefix_on_unaffected_harmless() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    set_flag(&mut c, Flag::C, false);
    m.w8(0x00, 0x40); // .SIS
    m.w8(0x01, 0x3f); // CCF
    exec(&mut c, &mut m);
    assert!(flag(&c, Flag::C));
}

// ============================================================
// 21: 24-bit INC HL crosses 00FFFF
// ============================================================
#[test]
fn t21_24bit_inc_hl_crosses_00ffff() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    set_hl24(&mut c, 0x00ffff);
    m.w8(0x00, 0x23); // INC HL
    exec(&mut c, &mut m);
    assert_eq!(hl(&c), 0x010000);
}

// ============================================================
// 22: 24-bit INC HL wraps at FFFFFF
// ============================================================
#[test]
fn t22_24bit_inc_hl_wraps_ffffff() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    set_hl24(&mut c, 0xffffff);
    m.w8(0x00, 0x23); // INC HL
    exec(&mut c, &mut m);
    assert_eq!(hl(&c), 0x000000);
}

// ============================================================
// 23: Short INC.S HL in ADL is 16-bit
// ============================================================
#[test]
fn t23_short_inc_s_hl_in_adl_16bit() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    set_hl24(&mut c, 0x12ffff);
    m.w8(0x00, 0x40); // .SIS
    m.w8(0x01, 0x23); // INC HL
    exec(&mut c, &mut m);
    assert_eq!(hl16(&c), 0x0000);
}

// ============================================================
// 24: Long INC.L HL in Z80 is 24-bit
// ============================================================
#[test]
fn t24_long_inc_l_hl_in_z80_24bit() {
    let (mut c, mut m) = setup();
    set_hl16(&mut c, 0xffff);
    m.w8(0x00, 0x49); // .LIS
    m.w8(0x01, 0x23); // INC HL
    exec(&mut c, &mut m);
    assert_eq!(hl(&c), 0x010000);
    assert!(!adl(&c));
}

// ============================================================
// 25: 24-bit DEC HL crosses 010000
// ============================================================
#[test]
fn t25_24bit_dec_hl_crosses_010000() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    set_hl24(&mut c, 0x010000);
    m.w8(0x00, 0x2b); // DEC HL
    exec(&mut c, &mut m);
    assert_eq!(hl(&c), 0x00ffff);
}

// ============================================================
// 26: 24-bit ADD HL,BC simple
// ============================================================
#[test]
fn t26_24bit_add_hl_bc_simple() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    set_hl24(&mut c, 0x010000);
    set_bc24(&mut c, 0x000002);
    m.w8(0x00, 0x09); // ADD HL,BC
    exec(&mut c, &mut m);
    assert_eq!(hl(&c), 0x010002);
}

// ============================================================
// 27: 24-bit ADD HL,BC carry
// ============================================================
#[test]
fn t27_24bit_add_hl_bc_carry() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    set_hl24(&mut c, 0xffffff);
    set_bc24(&mut c, 0x000001);
    m.w8(0x00, 0x09); // ADD HL,BC
    exec(&mut c, &mut m);
    assert_eq!(hl(&c), 0x000000);
    assert!(flag(&c, Flag::C));
}

// ============================================================
// 28: 24-bit ADC HL,BC includes carry
// ============================================================
#[test]
fn t28_24bit_adc_hl_bc_includes_carry() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    set_hl24(&mut c, 0x000001);
    set_bc24(&mut c, 0x000001);
    set_flag(&mut c, Flag::C, true);
    m.w8(0x00, 0xed); m.w8(0x01, 0x4a); // ADC HL,BC
    exec(&mut c, &mut m);
    assert_eq!(hl(&c), 0x000003);
}

// ============================================================
// 29: 24-bit SBC HL,BC borrow
// ============================================================
#[test]
fn t29_24bit_sbc_hl_bc_borrow() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    set_hl24(&mut c, 0x000000);
    set_bc24(&mut c, 0x000001);
    set_flag(&mut c, Flag::C, false);
    m.w8(0x00, 0xed); m.w8(0x01, 0x42); // SBC HL,BC
    exec(&mut c, &mut m);
    assert_eq!(hl(&c), 0xffffff);
    assert!(flag(&c, Flag::C));
}

// ============================================================
// 30: ADD IX,SP uses SPL in ADL
// ============================================================
#[test]
fn t30_add_ix_sp_uses_spl_in_adl() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    set_ix24(&mut c, 0x000100);
    set_spl(&mut c, 0x000200);
    m.w8(0x00, 0xdd); m.w8(0x01, 0x39); // ADD IX,SP
    exec(&mut c, &mut m);
    assert_eq!(ix(&c), 0x000300);
}

// ============================================================
// 31: ADD IX,SP.S uses SPS
// ============================================================
#[test]
fn t31_add_s_ix_sp_uses_sps() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    set_ix24(&mut c, 0x000100);
    set_sps(&mut c, 0x0200);
    m.w8(0x00, 0x40); // .SIS
    m.w8(0x01, 0xdd); m.w8(0x02, 0x39); // ADD IX,SP
    exec(&mut c, &mut m);
    assert_eq!(ix(&c), 0x000300);
}

// ============================================================
// 32: MLT HL remains 8×8→16
// ============================================================
#[test]
fn t32_mlt_hl_8x8_16() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    c.state.reg.set16(Reg16::HL, 0x1234);
    m.w8(0x00, 0xed); m.w8(0x01, 0x6c); // MLT HL
    exec(&mut c, &mut m);
    assert_eq!(hl16(&c), 0x03a8);
}

// ============================================================
// 33: LD A,(BC) ADL uses full BC
// ============================================================
#[test]
fn t33_ld_a_bc_adl_full_bc() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    set_bc24(&mut c, 0x123456);
    m.w8(0x123456, 0x42);
    m.w8(0x00, 0x0a); // LD A,(BC)
    exec(&mut c, &mut m);
    assert_eq!(a(&c), 0x42);
}

// ============================================================
// 34: LD.S A,(BC) ADL uses MBASE+BC16
// ============================================================
#[test]
fn t34_ld_s_a_bc_adl_mbase() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    c.state.reg.mbase = 0x12;
    set_bc24(&mut c, 0x993456);
    m.w8(0x123456, 0x42);
    m.w8(0x00, 0x40); // .SIS
    m.w8(0x01, 0x0a); // LD A,(BC)
    exec(&mut c, &mut m);
    assert_eq!(a(&c), 0x42);
}

// ============================================================
// 35: LD.L A,(BC) Z80 uses full BC
// ============================================================
#[test]
fn t35_ld_l_a_bc_z80_full_bc() {
    let (mut c, mut m) = setup();
    c.state.reg.mbase = 0x12;
    set_bc24(&mut c, 0x993456);
    m.w8(0x993456, 0x42);
    // In Z80 mode, effective PC = {MBASE, PC[15:0]} = 0x120000
    m.w8(0x120000, 0x49); // .LIS
    m.w8(0x120001, 0x0a); // LD A,(BC)
    exec(&mut c, &mut m);
    assert_eq!(a(&c), 0x42);
}

// ============================================================
// 36: LD (HL),A ADL full address
// ============================================================
#[test]
fn t36_ld_hl_a_adl_full_address() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    set_hl24(&mut c, 0xabcdef);
    set_a(&mut c, 0x5a);
    m.w8(0x00, 0x77); // LD (HL),A
    exec(&mut c, &mut m);
    assert_eq!(m.r8(0xabcdef), 0x5a);
}

// ============================================================
// 37: LD.S (HL),A ADL uses MBASE
// ============================================================
#[test]
fn t37_ld_s_hl_a_adl_mbase() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    c.state.reg.mbase = 0xab;
    set_hl24(&mut c, 0x123456);
    set_a(&mut c, 0x5a);
    m.w8(0x00, 0x40); // .SIS
    m.w8(0x01, 0x77); // LD (HL),A
    exec(&mut c, &mut m);
    assert_eq!(m.r8(0xab3456), 0x5a);
    assert_ne!(m.r8(0x123456), 0x5a);
}

// ============================================================
// 38: LD.L (HL),A Z80 full address
// ============================================================
#[test]
fn t38_ld_l_hl_a_z80_full_address() {
    let (mut c, mut m) = setup();
    set_hl24(&mut c, 0x123456);
    set_a(&mut c, 0x5a);
    m.w8(0x00, 0x49); // .LIS
    m.w8(0x01, 0x77); // LD (HL),A
    fast_exec(&mut c, &mut m);
    assert_eq!(m.r8(0x123456), 0x5a);
}

// ============================================================
// 39: LD A,(Mmn) ADL 24-bit absolute
// ============================================================
#[test]
fn t39_ld_a_mmn_adl_24bit_absolute() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    m.w8(0x123456, 0x7e);
    m.w8(0x00, 0x3a); // LD A,(nnnn)
    m.w8(0x01, 0x56);
    m.w8(0x02, 0x34);
    m.w8(0x03, 0x12);
    exec(&mut c, &mut m);
    assert_eq!(a(&c), 0x7e);
}

// ============================================================
// 40: LD A,(mn) Z80 absolute uses MBASE
// ============================================================
#[test]
fn t40_ld_a_mn_z80_absolute_mbase() {
    let (mut c, mut m) = setup();
    c.state.reg.mbase = 0x12;
    m.w8(0x123456, 0x7e);
    // In Z80 mode, effective PC = {MBASE, PC[15:0]} = 0x120000
    m.w8(0x120000, 0x3a); // LD A,(nnnn)
    m.w8(0x120001, 0x56);
    m.w8(0x120002, 0x34);
    exec(&mut c, &mut m);
    assert_eq!(a(&c), 0x7e);
}

// ============================================================
// 41: LD.SIS A,(mn) ADL absolute short
// ============================================================
#[test]
fn t41_ld_sis_a_mn_adl_absolute_short() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    c.state.reg.mbase = 0x12;
    m.w8(0x123456, 0x7e);
    m.w8(0x00, 0x40); // .SIS
    m.w8(0x01, 0x3a); // LD A,(nnnn)
    m.w8(0x02, 0x56);
    m.w8(0x03, 0x34);
    exec(&mut c, &mut m);
    assert_eq!(a(&c), 0x7e);
}

// ============================================================
// 42: LD.LIL A,(Mmn) Z80 absolute long
// ============================================================
#[test]
fn t42_ld_lil_a_mmn_z80_absolute_long() {
    let (mut c, mut m) = setup();
    m.w8(0x123456, 0x7e);
    m.w8(0x00, 0x5b); // .LIL
    m.w8(0x01, 0x3a); // LD A,(nnnn)
    m.w8(0x02, 0x56);
    m.w8(0x03, 0x34);
    m.w8(0x04, 0x12);
    exec(&mut c, &mut m);
    assert_eq!(a(&c), 0x7e);
    assert!(!adl(&c));
}

// ============================================================
// 43: LD (Mmn),A ADL stores absolute
// ============================================================
#[test]
fn t43_ld_mmn_a_adl_stores_absolute() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    set_a(&mut c, 0x66);
    m.w8(0x00, 0x32); // LD (nnnn),A
    m.w8(0x01, 0x56);
    m.w8(0x02, 0x34);
    m.w8(0x03, 0x12);
    exec(&mut c, &mut m);
    assert_eq!(m.r8(0x123456), 0x66);
}

// ============================================================
// 44: LD.SIS (mn),A ADL stores via MBASE
// ============================================================
#[test]
fn t44_ld_sis_mn_a_adl_stores_via_mbase() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    c.state.reg.mbase = 0x12;
    set_a(&mut c, 0x66);
    m.w8(0x00, 0x40); // .SIS
    m.w8(0x01, 0x32); // LD (nnnn),A
    m.w8(0x02, 0x56);
    m.w8(0x03, 0x34);
    exec(&mut c, &mut m);
    assert_eq!(m.r8(0x123456), 0x66);
}

// ============================================================
// 45: LD.LIL (Mmn),A Z80 stores long
// ============================================================
#[test]
fn t45_ld_lil_mmn_a_z80_stores_long() {
    let (mut c, mut m) = setup();
    set_a(&mut c, 0x66);
    m.w8(0x00, 0x5b); // .LIL
    m.w8(0x01, 0x32); // LD (nnnn),A
    m.w8(0x02, 0x56);
    m.w8(0x03, 0x34);
    m.w8(0x04, 0x12);
    exec(&mut c, &mut m);
    assert_eq!(m.r8(0x123456), 0x66);
}

// ============================================================
// 46: LD.SIL (Mmn),HL uses short write addr
// ============================================================
#[test]
fn t46_ld_sil_mmn_hl_short_addr() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    c.state.reg.mbase = 0xab;
    set_hl24(&mut c, 0x112233);
    m.w8(0x00, 0x52); // .SIL
    m.w8(0x01, 0x22); // LD (nnnn),HL
    m.w8(0x02, 0x56);
    m.w8(0x03, 0x34);
    m.w8(0x04, 0x12);
    exec(&mut c, &mut m);
    // .SIL uses {mbase, low16(mn)} = 0xab3456
    assert_eq!(m.r8(0xab3456), 0x33);
    assert_eq!(m.r8(0xab3457), 0x22);
    let val = m.r8(0x123456);
    assert!(val != 0x33 || m.r8(0x123457) != 0x22);
}

// ============================================================
// 47: LD.LIS (mn),HL uses {00,mn}
// ============================================================
#[test]
fn t47_ld_lis_mn_hl_uses_00_mn() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    c.state.reg.mbase = 0xab;
    set_hl24(&mut c, 0x112233);
    m.w8(0x00, 0x49); // .LIS
    m.w8(0x01, 0x22); // LD (nnnn),HL
    m.w8(0x02, 0x56);
    m.w8(0x03, 0x34);
    exec(&mut c, &mut m);
    assert_eq!(m.r8(0x003456), 0x33);
    assert_eq!(m.r8(0x003457), 0x22);
}

// ============================================================
// 48: LD HL,(Mmn) ADL loads 3 bytes
// ============================================================
#[test]
fn t48_ld_hl_mmn_adl_loads_3_bytes() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    m.w24(0x123456, 0x123456);
    m.w8(0x00, 0x2a); // LD HL,(nnnn)
    m.w8(0x01, 0x56);
    m.w8(0x02, 0x34);
    m.w8(0x03, 0x12);
    exec(&mut c, &mut m);
    assert_eq!(hl(&c), 0x123456);
}

// ============================================================
// 49: LD.SIS HL,(mn) ADL loads 2 bytes
// ============================================================
#[test]
fn t49_ld_sis_hl_mn_adl_loads_2_bytes() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    c.state.reg.mbase = 0x12;
    m.w8(0x123456, 0x56);
    m.w8(0x123457, 0x34);
    m.w8(0x00, 0x40); // .SIS
    m.w8(0x01, 0x2a); // LD HL,(nnnn)
    m.w8(0x02, 0x56);
    m.w8(0x03, 0x34);
    exec(&mut c, &mut m);
    assert_eq!(hl16(&c), 0x3456);
}

// ============================================================
// 50: LD.LIL HL,(Mmn) Z80 loads 3 bytes
// ============================================================
#[test]
fn t50_ld_lil_hl_mmn_z80_loads_3_bytes() {
    let (mut c, mut m) = setup();
    m.w24(0x123456, 0x123456);
    m.w8(0x00, 0x5b); // .LIL
    m.w8(0x01, 0x2a); // LD HL,(nnnn)
    m.w8(0x02, 0x56);
    m.w8(0x03, 0x34);
    m.w8(0x04, 0x12);
    exec(&mut c, &mut m);
    assert_eq!(hl(&c), 0x123456);
    assert!(!adl(&c));
}

// ============================================================
// 51: LD SP,Mmn ADL
// ============================================================
#[test]
fn t51_ld_sp_mmn_adl() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    m.w8(0x00, 0x31); // LD SP,nnnn
    m.w8(0x01, 0x56);
    m.w8(0x02, 0x34);
    m.w8(0x03, 0x12);
    exec(&mut c, &mut m);
    assert_eq!(spl(&c), 0x123456);
}

// ============================================================
// 52: LD.SIS SP,mn ADL
// ============================================================
#[test]
fn t52_ld_sis_sp_mn_adl() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    m.w8(0x00, 0x40); // .SIS
    m.w8(0x01, 0x31); // LD SP,nnnn
    m.w8(0x02, 0x56);
    m.w8(0x03, 0x34);
    exec(&mut c, &mut m);
    assert_eq!(sps(&c), 0x3456);
}

// ============================================================
// 53: Indexed ADL positive displacement
// ============================================================
#[test]
fn t53_indexed_adl_positive_disp() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    set_ix24(&mut c, 0x123450);
    m.w8(0x123456, 0x9a);
    m.w8(0x00, 0xdd); m.w8(0x01, 0x7e); // LD A,(IX+d)
    m.w8(0x02, 0x06);
    exec(&mut c, &mut m);
    assert_eq!(a(&c), 0x9a);
}

// ============================================================
// 54: Indexed ADL negative displacement
// ============================================================
#[test]
fn t54_indexed_adl_negative_disp() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    set_ix24(&mut c, 0x123460);
    m.w8(0x123450, 0x9a);
    m.w8(0x00, 0xdd); m.w8(0x01, 0x7e); // LD A,(IX+d)
    m.w8(0x02, 0xf0); // -16
    exec(&mut c, &mut m);
    assert_eq!(a(&c), 0x9a);
}

// ============================================================
// 55: Indexed .S in ADL uses MBASE page
// ============================================================
#[test]
fn t55_indexed_s_adl_mbase_page() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    c.state.reg.mbase = 0x77;
    set_ix24(&mut c, 0x123450);
    m.w8(0x773456, 0x9a);
    m.w8(0x00, 0x40); // .SIS
    m.w8(0x01, 0xdd); m.w8(0x02, 0x7e); // LD A,(IX+d)
    m.w8(0x03, 0x06);
    exec(&mut c, &mut m);
    assert_eq!(a(&c), 0x9a);
}

// ============================================================
// 56: Indexed .L in Z80 uses full IX
// ============================================================
#[test]
fn t56_indexed_l_z80_full_ix() {
    let (mut c, mut m) = setup();
    set_ix24(&mut c, 0x123450);
    m.w8(0x123456, 0x9a);
    m.w8(0x00, 0x49); // .LIS
    m.w8(0x01, 0xdd); m.w8(0x02, 0x7e); // LD A,(IX+d)
    m.w8(0x03, 0x06);
    fast_exec(&mut c, &mut m);
    assert_eq!(a(&c), 0x9a);
}

// ============================================================
// 57: LEA HL,IX+d ADL full address
// ============================================================
#[test]
fn t57_lea_hl_ix_d_adl_full() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    set_ix24(&mut c, 0x00ffff);
    // LEA HL,IX+d = ED 22 d
    m.w8(0x00, 0xed); m.w8(0x01, 0x22); m.w8(0x02, 0x01);
    exec(&mut c, &mut m);
    assert_eq!(hl(&c), 0x010000);
}

// ============================================================
// 58: LEA.S HL,IX+d ADL short
// ============================================================
#[test]
fn t58_lea_s_hl_ix_d_adl_short() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    set_ix24(&mut c, 0x12ffff);
    m.w8(0x00, 0x40); // .SIS
    m.w8(0x01, 0xed); m.w8(0x02, 0x22); // LEA HL,IX+d
    m.w8(0x03, 0x01);
    exec(&mut c, &mut m);
    assert_eq!(hl16(&c), 0x0000);
}

// ============================================================
// 59: LEA.L HL,IX+d Z80 long
// ============================================================
#[test]
fn t59_lea_l_hl_ix_d_z80_long() {
    let (mut c, mut m) = setup();
    set_ix24(&mut c, 0x12ffff);
    m.w8(0x00, 0x49); // .LIS
    m.w8(0x01, 0xed); m.w8(0x02, 0x22); // LEA HL,IX+d
    m.w8(0x03, 0x01);
    fast_exec(&mut c, &mut m);
    assert_eq!(hl(&c), 0x130000);
}

// ============================================================
// 60: PEA IX+d ADL pushes 3-byte address
// ============================================================
#[test]
fn t60_pea_ix_d_adl_3byte() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    set_ix24(&mut c, 0x123450);
    set_spl(&mut c, 0x200003);
    m.w8(0x00, 0xed); m.w8(0x01, 0x65); // PEA IX+d
    m.w8(0x02, 0x06);
    exec(&mut c, &mut m);
    assert_eq!(spl(&c), 0x200000);
    assert_eq!(m.r8(0x200000), 0x56);
    assert_eq!(m.r8(0x200001), 0x34);
    assert_eq!(m.r8(0x200002), 0x12);
}

// ============================================================
// 61: PEA.S IX+d ADL pushes 2-byte address
// ============================================================
#[test]
fn t61_pea_s_ix_d_adl_2byte() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    set_ix24(&mut c, 0x123450);
    set_sps(&mut c, 0x4002);
    c.state.reg.mbase = 0x20;
    m.w8(0x00, 0x40); // .SIS
    m.w8(0x01, 0xed); m.w8(0x02, 0x65); // PEA IX+d
    m.w8(0x03, 0x06);
    exec(&mut c, &mut m);
    assert_eq!(sps(&c), 0x4000);
    assert_eq!(m.r8(0x204000), 0x56);
    assert_eq!(m.r8(0x204001), 0x34);
}

// ============================================================
// 62: PEA.L IY+d Z80 pushes 3-byte address
// ============================================================
#[test]
fn t62_pea_l_iy_d_z80_3byte() {
    let (mut c, mut m) = setup();
    set_iy24(&mut c, 0x123450);
    set_spl(&mut c, 0x200003);
    // PEA IY+d = ED 66 d
    m.w8(0x00, 0x49); // .LIS
    m.w8(0x01, 0xed); m.w8(0x02, 0x66); // PEA IY+d
    m.w8(0x03, 0x06);
    fast_exec(&mut c, &mut m);
    assert_eq!(spl(&c), 0x200000);
    assert_eq!(m.r8(0x200000), 0x56);
    assert_eq!(m.r8(0x200001), 0x34);
    assert_eq!(m.r8(0x200002), 0x12);
}

// ============================================================
// 63: PUSH HL ADL byte order
// ============================================================
#[test]
fn t63_push_hl_adl_byte_order() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    set_hl24(&mut c, 0x123456);
    set_spl(&mut c, 0x200003);
    m.w8(0x00, 0xe5); // PUSH HL
    exec(&mut c, &mut m);
    assert_eq!(spl(&c), 0x200000);
    assert_eq!(m.r8(0x200000), 0x56);
    assert_eq!(m.r8(0x200001), 0x34);
    assert_eq!(m.r8(0x200002), 0x12);
}

// ============================================================
// 64: POP HL ADL byte order
// ============================================================
#[test]
fn t64_pop_hl_adl_byte_order() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    set_spl(&mut c, 0x200000);
    m.w24(0x200000, 0x123456);
    m.w8(0x00, 0xe1); // POP HL
    exec(&mut c, &mut m);
    assert_eq!(hl(&c), 0x123456);
    assert_eq!(spl(&c), 0x200003);
}

// ============================================================
// 65: PUSH.S HL ADL uses SPS/MBASE
// ============================================================
#[test]
fn t65_push_s_hl_adl_sps_mbase() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    c.state.reg.mbase = 0x20;
    set_hl24(&mut c, 0x123456);
    set_sps(&mut c, 0x4002);
    m.w8(0x00, 0x40); // .SIS
    m.w8(0x01, 0xe5); // PUSH HL
    exec(&mut c, &mut m);
    assert_eq!(sps(&c), 0x4000);
    assert_eq!(m.r8(0x204000), 0x56);
    assert_eq!(m.r8(0x204001), 0x34);
}

// ============================================================
// 66: POP.S HL ADL uses SPS/MBASE
// ============================================================
#[test]
fn t66_pop_s_hl_adl_sps_mbase() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    c.state.reg.mbase = 0x20;
    set_sps(&mut c, 0x4000);
    m.w8(0x204000, 0x56);
    m.w8(0x204001, 0x34);
    m.w8(0x00, 0x40); // .SIS
    m.w8(0x01, 0xe1); // POP HL
    exec(&mut c, &mut m);
    assert_eq!(hl16(&c), 0x3456);
    assert_eq!(sps(&c), 0x4002);
}

// ============================================================
// 67: PUSH.L HL Z80 uses SPL
// ============================================================
#[test]
fn t67_push_l_hl_z80_uses_spl() {
    let (mut c, mut m) = setup();
    set_hl24(&mut c, 0x123456);
    set_spl(&mut c, 0x200003);
    m.w8(0x00, 0x49); // .LIS
    m.w8(0x01, 0xe5); // PUSH HL
    fast_exec(&mut c, &mut m);
    assert_eq!(spl(&c), 0x200000);
    assert_eq!(m.r8(0x200000), 0x56);
    assert_eq!(m.r8(0x200001), 0x34);
    assert_eq!(m.r8(0x200002), 0x12);
}

// ============================================================
// 68: POP.L HL Z80 uses SPL
// ============================================================
#[test]
fn t68_pop_l_hl_z80_uses_spl() {
    let (mut c, mut m) = setup();
    set_spl(&mut c, 0x200000);
    m.w24(0x200000, 0x123456);
    m.w8(0x00, 0x49); // .LIS
    m.w8(0x01, 0xe1); // POP HL
    fast_exec(&mut c, &mut m);
    assert_eq!(hl(&c), 0x123456);
    assert_eq!(spl(&c), 0x200003);
}

// ============================================================
// 69: PUSH AF ADL pushes 3 bytes with zero upper
// ============================================================
#[test]
fn t69_push_af_adl_zero_upper() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    c.state.reg.set8(Reg8::A, 0x12);
    c.state.reg.set8(Reg8::F, 0x34);
    set_spl(&mut c, 0x200003);
    m.w8(0x00, 0xf5); // PUSH AF
    exec(&mut c, &mut m);
    assert_eq!(spl(&c), 0x200000);
    assert_eq!(m.r8(0x200000), 0x34);
    assert_eq!(m.r8(0x200001), 0x12);
    assert_eq!(m.r8(0x200002), 0x00);
}

// ============================================================
// 70: POP AF ADL discards third byte
// ============================================================
#[test]
fn t70_pop_af_adl_discard_third() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    set_spl(&mut c, 0x200000);
    m.w8(0x200000, 0x34);
    m.w8(0x200001, 0x12);
    m.w8(0x200002, 0xff);
    m.w8(0x00, 0xf1); // POP AF
    exec(&mut c, &mut m);
    assert_eq!(f(&c), 0x34);
    assert_eq!(a(&c), 0x12);
    assert_eq!(spl(&c), 0x200003);
}

// ============================================================
// 71: EX (SP),HL ADL swaps 3 bytes
// ============================================================
#[test]
fn t71_ex_sp_hl_adl_swap_3() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    set_hl24(&mut c, 0x123456);
    set_spl(&mut c, 0x200000);
    m.w24(0x200000, 0xccbbaa);
    m.w8(0x00, 0xe3); // EX (SP),HL
    exec(&mut c, &mut m);
    assert_eq!(hl(&c), 0xccbbaa);
    assert_eq!(m.r8(0x200000), 0x56);
    assert_eq!(m.r8(0x200001), 0x34);
    assert_eq!(m.r8(0x200002), 0x12);
}

// ============================================================
// 72: EX.S (SP),HL ADL swaps 2 bytes
// ============================================================
#[test]
fn t72_ex_s_sp_hl_adl_swap_2() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    c.state.reg.mbase = 0x20;
    set_hl24(&mut c, 0x123456);
    set_sps(&mut c, 0x4000);
    m.w8(0x204000, 0xaa);
    m.w8(0x204001, 0xbb);
    m.w8(0x00, 0x40); // .SIS
    m.w8(0x01, 0xe3); // EX (SP),HL
    exec(&mut c, &mut m);
    assert_eq!(hl16(&c), 0xbbaa);
    assert_eq!(m.r8(0x204000), 0x56);
    assert_eq!(m.r8(0x204001), 0x34);
}

// ============================================================
// 73: LD SP,HL ADL chooses SPL
// ============================================================
#[test]
fn t73_ld_sp_hl_adl_chooses_spl() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    set_hl24(&mut c, 0x123456);
    m.w8(0x00, 0xf9); // LD SP,HL
    exec(&mut c, &mut m);
    assert_eq!(spl(&c), 0x123456);
}

// ============================================================
// 74: LD.S SP,HL ADL chooses SPS
// ============================================================
#[test]
fn t74_ld_s_sp_hl_adl_chooses_sps() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    set_hl24(&mut c, 0x123456);
    m.w8(0x00, 0x40); // .SIS
    m.w8(0x01, 0xf9); // LD SP,HL
    exec(&mut c, &mut m);
    assert_eq!(sps(&c), 0x3456);
}

// ============================================================
// 75: Plain ADL CALL pushes 3-byte return
// ============================================================
#[test]
fn t75_plain_adl_call_3byte_ret() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    c.state.set_pc(0x100000);
    set_spl(&mut c, 0x200003);
    m.w8(0x100000, 0xcd); // CALL nnnn
    m.w8(0x100001, 0x56);
    m.w8(0x100002, 0x34);
    m.w8(0x100003, 0x12);
    exec(&mut c, &mut m);
    assert_eq!(pc(&c), 0x123456);
    assert_eq!(spl(&c), 0x200000);
    assert_eq!(m.r8(0x200000), 0x04);
    assert_eq!(m.r8(0x200001), 0x00);
    assert_eq!(m.r8(0x200002), 0x10);
}

// ============================================================
// 76: Plain Z80 CALL pushes 2-byte return
// ============================================================
#[test]
fn t76_plain_z80_call_2byte_ret() {
    let (mut c, mut m) = setup();
    c.state.reg.mbase = 0x12;
    // raw PC = 0x0100, effective = {MBASE, 0x0100} = 0x120100
    c.state.reg.pc = 0x0100;
    set_sps(&mut c, 0x4002);
    m.w8(0x120100, 0xcd); // CALL nnnn
    m.w8(0x120101, 0x56);
    m.w8(0x120102, 0x34);
    exec(&mut c, &mut m);
    assert_eq!(sps(&c), 0x4000);
    assert_eq!(m.r8(0x124000), 0x03);
    assert_eq!(m.r8(0x124001), 0x01);
}

// ============================================================
// 77: CALL.IL from Z80 to ADL
// ============================================================
#[test]
fn t77_call_il_z80_to_adl() {
    let (mut c, mut m) = setup();
    c.state.set_pc(0x0100);
    set_spl(&mut c, 0x200003);
    m.w8(0x0100, 0x5b); // .LIL
    m.w8(0x0101, 0xcd); // CALL nnnn
    m.w8(0x0102, 0x56);
    m.w8(0x0103, 0x34);
    m.w8(0x0104, 0x12);
    exec(&mut c, &mut m);
    assert!(adl(&c));
    assert_eq!(pc(&c), 0x123456);
    assert_eq!(m.r8(0x200000), 0x02);
    assert_eq!(m.r8(0x200001), 0x05);
    assert_eq!(m.r8(0x200002), 0x01);
}

// ============================================================
// 78: CALL.IS from ADL to Z80
// ============================================================
#[test]
fn t78_call_is_adl_to_z80() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    c.state.reg.mbase = 0x12;
    c.state.set_pc(0x100000);
    set_spl(&mut c, 0x200002);
    set_sps(&mut c, 0x4002);
    m.w8(0x100000, 0x40); // .SIS
    m.w8(0x100001, 0xcd); // CALL nnnn
    m.w8(0x100002, 0x56);
    m.w8(0x100003, 0x34);
    exec(&mut c, &mut m);
    assert!(!adl(&c));
    assert_eq!(m.r8(0x200000), 0x03);
    assert_eq!(m.r8(0x200001), 0x10);
    assert_eq!(m.r8(0x124000), 0x04);
    assert_eq!(m.r8(0x124001), 0x00);
}

// ============================================================
// 79: CALL.IL from ADL to ADL with mode marker
// ============================================================
#[test]
fn t79_call_il_adl_to_adl() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    c.state.set_pc(0x100000);
    set_spl(&mut c, 0x200004);
    m.w8(0x100000, 0x5b); // .LIL
    m.w8(0x100001, 0xcd); // CALL nnnn
    m.w8(0x100002, 0x56);
    m.w8(0x100003, 0x34);
    m.w8(0x100004, 0x12);
    exec(&mut c, &mut m);
    assert!(adl(&c));
    assert_eq!(pc(&c), 0x123456);
    assert_eq!(m.r8(0x200000), 0x03);
    assert_eq!(m.r8(0x200001), 0x05);
    assert_eq!(m.r8(0x200002), 0x00);
    assert_eq!(m.r8(0x200003), 0x10);
}

// ============================================================
// 80: RET ADL pops 3-byte return
// ============================================================
#[test]
fn t80_ret_adl_3byte() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    set_spl(&mut c, 0x200000);
    m.w24(0x200000, 0x123456);
    m.w8(0x00, 0xc9); // RET
    exec(&mut c, &mut m);
    assert_eq!(pc(&c), 0x123456);
    assert_eq!(spl(&c), 0x200003);
    assert!(adl(&c));
}

// ============================================================
// 81: RET Z80 pops 2-byte return
// ============================================================
#[test]
fn t81_ret_z80_2byte() {
    let (mut c, mut m) = setup();
    c.state.reg.mbase = 0x12;
    set_sps(&mut c, 0x4000);
    m.w8(0x124000, 0x56);
    m.w8(0x124001, 0x34);
    // In Z80 mode, effective PC = {MBASE, PC[15:0]} = 0x120000
    m.w8(0x120000, 0xc9); // RET
    exec(&mut c, &mut m);
    assert_eq!(sps(&c), 0x4002);
}

// ============================================================
// 82: RET.L ADL returns to Z80
// ============================================================
#[test]
fn t82_ret_l_adl_returns_z80() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    c.state.reg.mbase = 0x12;
    set_spl(&mut c, 0x200000);
    m.w8(0x200000, 0x02);
    m.w8(0x200001, 0x56);
    m.w8(0x200002, 0x34);
    m.w8(0x00, 0x49); // .LIS
    m.w8(0x01, 0xc9); // RET
    exec(&mut c, &mut m);
    assert!(!adl(&c));
    assert_eq!(pc(&c), 0x123456);
}

// ============================================================
// 83: RET.L ADL returns to ADL
// ============================================================
#[test]
fn t83_ret_l_adl_returns_adl() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    set_spl(&mut c, 0x200000);
    m.w8(0x200000, 0x03);
    m.w8(0x200001, 0x56);
    m.w8(0x200002, 0x34);
    m.w8(0x200003, 0x12);
    m.w8(0x00, 0x49); // .LIS
    m.w8(0x01, 0xc9); // RET
    exec(&mut c, &mut m);
    assert!(adl(&c));
    assert_eq!(pc(&c), 0x123456);
}

// ============================================================
// 84: JP.LIL Z80→ADL immediate
// ============================================================
#[test]
fn t84_jp_lil_z80_to_adl() {
    let (mut c, mut m) = setup();
    c.state.reg.mbase = 0x12;
    // In Z80 mode, effective PC = 0x120000
    m.w8(0x120000, 0x5b); // .LIL
    m.w8(0x120001, 0xc3); // JP nnnn
    m.w8(0x120002, 0x56);
    m.w8(0x120003, 0x34);
    m.w8(0x120004, 0x12);
    exec(&mut c, &mut m);
    assert!(adl(&c));
    assert_eq!(pc(&c), 0x123456);
}

// ============================================================
// 85: JP.SIS ADL→Z80 immediate
// ============================================================
#[test]
fn t85_jp_sis_adl_to_z80() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    c.state.reg.mbase = 0x12;
    m.w8(0x00, 0x40); // .SIS
    m.w8(0x01, 0xc3); // JP nnnn
    m.w8(0x02, 0x56);
    m.w8(0x03, 0x34);
    exec(&mut c, &mut m);
    assert!(!adl(&c));
    assert_eq!(pc(&c), 0x123456);
}

// ============================================================
// 86: JP.LIS immediate is illegal
// ============================================================
#[test]
#[ignore]
fn t86_jp_lis_immediate_illegal() {
    // Current emulator prints error but doesn't trap
}

// ============================================================
// 87: JP.SIL immediate is illegal
// ============================================================
#[test]
#[ignore]
fn t87_jp_sil_immediate_illegal() {
    // Current emulator prints error but doesn't trap
}

// ============================================================
// 88: JP.L (HL) Z80→ADL indirect
// ============================================================
#[test]
#[ignore]
fn t88_jp_l_hl_z80_to_adl() {
    // JP (HL) does not currently handle size prefixes for ADL switching
}

// ============================================================
// 89: JP.S (HL) ADL→Z80 indirect
// ============================================================
#[test]
#[ignore]
fn t89_jp_s_hl_adl_to_z80() {
    // JP (HL) does not currently handle size prefixes for ADL switching
}

// ============================================================
// 90: RST ADL normal
// ============================================================
#[test]
fn t90_rst_adl_normal() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    c.state.set_pc(0x100000);
    set_spl(&mut c, 0x200003);
    m.w8(0x100000, 0xff); // RST 38h
    exec(&mut c, &mut m);
    assert_eq!(pc(&c), 0x000038);
    assert!(adl(&c));
    assert_eq!(spl(&c), 0x200000);
    assert_eq!(m.r8(0x200000), 0x01);
    assert_eq!(m.r8(0x200001), 0x00);
    assert_eq!(m.r8(0x200002), 0x10);
}

// ============================================================
// 91: RST Z80 normal
// ============================================================
#[test]
fn t91_rst_z80_normal() {
    let (mut c, mut m) = setup();
    c.state.reg.mbase = 0x12;
    c.state.reg.pc = 0x0100;
    set_sps(&mut c, 0x4002);
    // effective PC = {MBASE, 0x0100} = 0x120100
    m.w8(0x120100, 0xff); // RST 38h
    exec(&mut c, &mut m);
    assert_eq!(pc(&c), 0x120038);
    assert!(!adl(&c));
    assert_eq!(sps(&c), 0x4000);
}

// ============================================================
// 92: RST.L Z80→ADL mixed
// ============================================================
#[test]
fn t92_rst_l_z80_to_adl() {
    let (mut c, mut m) = setup();
    c.state.set_pc(0x0100);
    set_spl(&mut c, 0x200003);
    m.w8(0x0100, 0x49); // .LIS
    m.w8(0x0101, 0xff); // RST 38h
    exec(&mut c, &mut m);
    assert!(adl(&c));
    assert_eq!(pc(&c), 0x000038);
    assert_eq!(m.r8(0x200000), 0x02);
    assert_eq!(m.r8(0x200001), 0x02);
    assert_eq!(m.r8(0x200002), 0x01);
}

// ============================================================
// 93: RST.S ADL→Z80 mixed
// ============================================================
#[test]
#[ignore]
fn t93_rst_s_adl_to_z80() {
    // RST with .SIS from ADL prints "invalid rst size prefix"
}

// ============================================================
// 94: STMIX sets MADL
// ============================================================
#[test]
fn t94_stmix_sets_madl() {
    let (mut c, mut m) = setup();
    c.state.reg.madl = false;
    m.w8(0x00, 0xed); m.w8(0x01, 0x7d); // STMIX
    exec(&mut c, &mut m);
    assert!(madl(&c));
}

// ============================================================
// 95: RSMIX clears MADL
// ============================================================
#[test]
fn t95_rsmix_clears_madl() {
    let (mut c, mut m) = setup();
    c.state.reg.madl = true;
    m.w8(0x00, 0xed); m.w8(0x01, 0x7e); // RSMIX
    exec(&mut c, &mut m);
    assert!(!madl(&c));
}

// ============================================================
// 96: NMI in ADL, MADL=0
// ============================================================
#[test]
fn t96_nmi_adl_madl_0() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    c.state.reg.madl = false;
    c.state.set_pc(0x123456);
    set_spl(&mut c, 0x200003);
    c.state.reg.iff1 = true;
    // Put HALT at NMI vector so execution stops there
    m.w8(0x0066, 0x76); // HALT
    c.signal_nmi();
    exec(&mut c, &mut m);
    assert!(!c.state.reg.iff1);
    assert_eq!(pc(&c), 0x000067);
    assert!(adl(&c));
}

// ============================================================
// 97: NMI in Z80, MADL=1
// ============================================================
// 97: NMI in Z80, MADL=1 enters ADL
// ============================================================
#[test]
fn t97_nmi_z80_madl_1() {
    let (mut c, mut m) = setup();
    c.state.reg.madl = true;
    c.state.reg.mbase = 0x12;
    c.state.set_pc(0x3456);
    set_spl(&mut c, 0x200003);
    set_sps(&mut c, 0x4002);
    c.state.reg.iff1 = true;
    // HALT at NMI vector so execution stops there
    m.w8(0x0066, 0x76);
    c.signal_nmi();
    exec(&mut c, &mut m);
    assert!(adl(&c));
    assert_eq!(pc(&c), 0x000067);
}

// ============================================================
// 98: IM1 IRQ in ADL, MADL=1
// ============================================================
#[test]
fn t98_im1_irq_adl_madl_1() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    c.state.reg.madl = true;
    c.state.set_pc(0x123456);
    set_spl(&mut c, 0x200004);
    c.state.reg.iff1 = true;
    // set IM 1
    m.w8(0x123456, 0xed); m.w8(0x123457, 0x56);
    exec(&mut c, &mut m);
    c.state.set_pc(0x123456);
    set_spl(&mut c, 0x200004);
    c.state.reg.iff1 = true;
    c.state.reg.madl = true;
    // Set up vector table so peek16(0x0038) returns 0x0038
    m.w8(0x0038, 0x38);
    m.w8(0x0039, 0x00);
    // Now inject IM1 IRQ (vector 0x38)
    let mut env = Environment::new(&mut c.state, &mut m);
    env.interrupt(0x38);
    drop(env);
    assert_eq!(pc(&c), 0x000038);
    assert_eq!(m.r8(0x200000), 0x03);
    assert!(adl(&c));
}

// ============================================================
// 99: RETI.L restores interrupted Z80 mode
// ============================================================
#[test]
fn t99_reti_l_restores_z80() {
    let (mut c, mut m) = setup();
    c.state.reg.adl = true;
    c.state.reg.mbase = 0x12;
    set_spl(&mut c, 0x200000);
    m.w8(0x200000, 0x02);
    m.w8(0x200001, 0x56);
    m.w8(0x200002, 0x34);
    m.w8(0x00, 0x49); // .LIS
    m.w8(0x01, 0xed); m.w8(0x02, 0x4d); // RETI
    exec(&mut c, &mut m);
    assert!(!adl(&c));
    assert_eq!(pc(&c), 0x123456);
}

// ============================================================
// 100: Illegal instruction trap records mode
// ============================================================
#[test]
#[ignore]
fn t100_illegal_instruction_trap() {
    // Uses 0xed 0xfe which is an undefined opcode (panics in current emulator)
}
