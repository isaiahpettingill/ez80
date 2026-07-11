use ez80::*;

fn run(code: &[u8], cpu: &mut Cpu, machine: &mut PlainMachine) {
    for (i, byte) in code.iter().copied().enumerate() {
        machine.poke(i as u32, byte);
    }
    cpu.state.set_pc(0);
    cpu.execute_instruction(machine);
}

#[test]
fn constructors_select_clear_modes() {
    assert_eq!(CpuMode::I8080, Cpu::new_8080().mode());
    assert_eq!(CpuMode::I8085, Cpu::new_8085().mode());
    assert_eq!(CpuMode::Z80, Cpu::new_z80().mode());
    assert_eq!(CpuMode::Z80N, Cpu::new_z80n().mode());
    assert_eq!(CpuMode::Z180, Cpu::new_z180().mode());
    assert_eq!(CpuMode::EZ80, Cpu::new_ez80().mode());
    assert_eq!(CpuMode::GameBoy, Cpu::new_gameboy().mode());

    for mode in [
        CpuMode::I8080,
        CpuMode::I8085,
        CpuMode::Z80,
        CpuMode::Z80N,
        CpuMode::Z180,
        CpuMode::EZ80,
        CpuMode::GameBoy,
    ] {
        assert_eq!(mode, Cpu::new_for_mode(mode).mode());
    }
}

#[test]
fn i8080_and_i8085_run_8080_base_opcodes() {
    for mode in [CpuMode::I8080, CpuMode::I8085] {
        let mut cpu = Cpu::new_for_mode(mode);
        let mut machine = PlainMachine::new();
        run(&[0x3c], &mut cpu, &mut machine); // INR A
        assert_eq!(0x00, cpu.registers().a());
    }
}

#[test]
fn z80n_runs_next_extensions() {
    let mut cpu = Cpu::new_z80n();
    let mut machine = PlainMachine::new();
    cpu.registers().set_a(0xa5);
    run(&[0xed, 0x23], &mut cpu, &mut machine); // SWAPNIB
    assert_eq!(0x5a, cpu.registers().a());

    run(&[0xed, 0x24], &mut cpu, &mut machine); // MIRROR A
    assert_eq!(0x5a, cpu.registers().a());

    cpu.registers().set8(Reg8::D, 6);
    cpu.registers().set8(Reg8::E, 7);
    run(&[0xed, 0x30], &mut cpu, &mut machine); // MUL D,E
    assert_eq!(42, cpu.registers().get16(Reg16::DE));

    cpu.registers().set_a(5);
    cpu.registers().set16(Reg16::HL, 0x1000);
    run(&[0xed, 0x31], &mut cpu, &mut machine); // ADD HL,A
    assert_eq!(0x1005, cpu.registers().get16(Reg16::HL));

    run(&[0xed, 0x34, 0x34, 0x12], &mut cpu, &mut machine); // ADD HL,$1234
    assert_eq!(0x2239, cpu.registers().get16(Reg16::HL));

    cpu.registers().set_a(0x99);
    run(&[0xed, 0x92, 0x12], &mut cpu, &mut machine); // NEXTREG $12,A
    assert_eq!(0x12, machine.port_in(0x243b));
    assert_eq!(0x99, machine.port_in(0x253b));

    run(&[0xed, 0x91, 0x34, 0x56], &mut cpu, &mut machine); // NEXTREG $34,$56
    assert_eq!(0x34, machine.port_in(0x243b));
    assert_eq!(0x56, machine.port_in(0x253b));
}

#[test]
fn z80n_runs_remaining_next_extensions() {
    let mut cpu = Cpu::new_z80n();
    let mut machine = PlainMachine::new();

    cpu.registers().set8(Reg8::B, 4);
    cpu.registers().set16(Reg16::DE, 0x1234);
    run(&[0xed, 0x28], &mut cpu, &mut machine); // BSLA DE,B
    assert_eq!(0x2340, cpu.registers().get16(Reg16::DE));

    cpu.registers().set8(Reg8::B, 1);
    cpu.registers().set16(Reg16::DE, 0x8001);
    run(&[0xed, 0x29], &mut cpu, &mut machine); // BSRA DE,B
    assert_eq!(0xc000, cpu.registers().get16(Reg16::DE));

    cpu.registers().set16(Reg16::SP, 0x2000);
    run(&[0xed, 0x8a, 0x12, 0x34], &mut cpu, &mut machine); // PUSH $1234
    assert_eq!(0x1ffe, cpu.registers().get16(Reg16::SP));
    assert_eq!(0x34, machine.peek(0x1ffe));
    assert_eq!(0x12, machine.peek(0x1fff));

    cpu.registers().set16(Reg16::BC, 0x7720);
    cpu.registers().set16(Reg16::HL, 0x3000);
    machine.poke(0x3000, 0x5a);
    run(&[0xed, 0x90], &mut cpu, &mut machine); // OUTINB
    assert_eq!(0x5a, machine.port_in(0x7720));
    assert_eq!(0x77, cpu.registers().get8(Reg8::B));
    assert_eq!(0x21, cpu.registers().get8(Reg8::C));
    assert_eq!(0x3001, cpu.registers().get16(Reg16::HL));

    cpu.registers().set16(Reg16::HL, 0x4000);
    run(&[0xed, 0x93], &mut cpu, &mut machine); // PIXELDN
    assert_eq!(0x4100, cpu.registers().get16(Reg16::HL));

    cpu.registers().set8(Reg8::D, 64);
    cpu.registers().set8(Reg8::E, 16);
    run(&[0xed, 0x94], &mut cpu, &mut machine); // PIXELAD
    assert_eq!(0x4802, cpu.registers().get16(Reg16::HL));

    run(&[0xed, 0x95], &mut cpu, &mut machine); // SETAE
    assert_eq!(0x80, cpu.registers().a());

    cpu.registers().set16(Reg16::BC, 0x0012);
    machine.port_out(0x0012, 0x03);
    run(&[0xed, 0x98], &mut cpu, &mut machine); // JP (C)
    assert_eq!(0x00c0, cpu.state.pc());
}

#[test]
fn z80n_runs_extended_block_copy_opcodes() {
    let mut cpu = Cpu::new_z80n();
    let mut machine = PlainMachine::new();

    cpu.registers().set_a(0x00);
    cpu.registers().set16(Reg16::HL, 0x2100);
    cpu.registers().set16(Reg16::DE, 0x2200);
    cpu.registers().set16(Reg16::BC, 1);
    machine.poke(0x2100, 0x66);
    run(&[0xed, 0xa4], &mut cpu, &mut machine); // LDIX
    assert_eq!(0x66, machine.peek(0x2200));
    assert_eq!(0x2101, cpu.registers().get16(Reg16::HL));
    assert_eq!(0x2201, cpu.registers().get16(Reg16::DE));
    assert_eq!(0, cpu.registers().get16(Reg16::BC));

    cpu.registers().set16(Reg16::HL, 0x2101);
    cpu.registers().set16(Reg16::DE, 0x2201);
    cpu.registers().set8(Reg8::L, 0x01);
    cpu.registers().set8(Reg8::D, 0x22);
    machine.poke(0x2101, 0x77);
    run(&[0xed, 0xa5], &mut cpu, &mut machine); // LDWS
    assert_eq!(0x77, machine.peek(0x2201));
    assert_eq!(0x02, cpu.registers().get8(Reg8::L));
    assert_eq!(0x23, cpu.registers().get8(Reg8::D));

    cpu.registers().set_a(0xff);
    cpu.registers().set16(Reg16::HL, 0x2103);
    cpu.registers().set16(Reg16::DE, 0x2203);
    cpu.registers().set16(Reg16::BC, 1);
    machine.poke(0x2103, 0x88);
    run(&[0xed, 0xac], &mut cpu, &mut machine); // LDDX
    assert_eq!(0x88, machine.peek(0x2203));
    assert_eq!(0x2102, cpu.registers().get16(Reg16::HL));
    assert_eq!(0x2204, cpu.registers().get16(Reg16::DE));

    cpu.registers().set16(Reg16::HL, 0x2300);
    cpu.registers().set16(Reg16::DE, 0x2402);
    cpu.registers().set16(Reg16::BC, 1);
    machine.poke(0x2302, 0x99);
    run(&[0xed, 0xb7], &mut cpu, &mut machine); // LDPIRX
    assert_eq!(0x99, machine.peek(0x2402));
    assert_eq!(0x2300, cpu.registers().get16(Reg16::HL));
    assert_eq!(0x2403, cpu.registers().get16(Reg16::DE));
}

#[test]
fn i8085_runs_added_rim_sim_opcodes() {
    let mut cpu = Cpu::new_8085();
    let mut machine = PlainMachine::new();
    cpu.registers().set_a(0x0d); // enable mask update, mask RST5.5 and RST7.5
    run(&[0x30], &mut cpu, &mut machine); // SIM
    assert_eq!(0x05, cpu.state.i8085_interrupt_mask);

    cpu.state.i8085_pending_interrupts = 0x03;
    cpu.state.i8085_serial_input = true;
    run(&[0x20], &mut cpu, &mut machine); // RIM
    assert_eq!(0xb5, cpu.registers().a());

    let mut cpu_8080 = Cpu::new_8080();
    cpu_8080.registers().set_a(0xaa);
    run(&[0x20], &mut cpu_8080, &mut machine); // 8080 keeps opcode $20 as NOP
    assert_eq!(0xaa, cpu_8080.registers().a());
}

#[test]
fn z80_keeps_next_extensions_disabled() {
    let mut cpu = Cpu::new_z80();
    let mut machine = PlainMachine::new();
    cpu.registers().set_a(0xa5);
    run(&[0xed, 0x23], &mut cpu, &mut machine);
    assert_eq!(0xa5, cpu.registers().a());
}

#[test]
fn z180_runs_z180_extended_opcodes() {
    let mut cpu = Cpu::new_z180();
    let mut machine = PlainMachine::new();
    run(&[0x40], &mut cpu, &mut machine); // LD B,B, not an eZ80 size prefix
    assert_eq!(1, cpu.state.pc());

    machine.port_out(0x12, 0x5a);
    run(&[0xed, 0x00, 0x12], &mut cpu, &mut machine); // IN0 B,($12)
    assert_eq!(0x5a, cpu.registers().get8(Reg8::B));

    cpu.registers().set8(Reg8::C, 0xa5);
    run(&[0xed, 0x09, 0x34], &mut cpu, &mut machine); // OUT0 ($34),C
    assert_eq!(0xa5, machine.port_in(0x34));

    cpu.registers().set_a(0xf0);
    cpu.registers().set8(Reg8::B, 0x0f);
    run(&[0xed, 0x04], &mut cpu, &mut machine); // TST A,B
    assert!(cpu.registers().get_flag(Flag::Z));

    machine.port_out(0xa5, 0xf0);
    cpu.registers().set8(Reg8::C, 0xa5);
    run(&[0xed, 0x74, 0x0f], &mut cpu, &mut machine); // TSTIO $0f
    assert!(cpu.registers().get_flag(Flag::Z));

    cpu.registers().set16(Reg16::BC, 0x0304);
    run(&[0xed, 0x4c], &mut cpu, &mut machine); // MLT BC
    assert_eq!(12, cpu.registers().get16(Reg16::BC));

    cpu.registers().set8(Reg8::B, 2);
    cpu.registers().set8(Reg8::C, 0x20);
    cpu.registers().set16(Reg16::HL, 0x0100);
    machine.poke(0x0100, 0x77);
    run(&[0xed, 0x83], &mut cpu, &mut machine); // OTIM
    assert_eq!(0x77, machine.port_in(0x20));
    assert_eq!(1, cpu.registers().get8(Reg8::B));
    assert_eq!(0x21, cpu.registers().get8(Reg8::C));
    assert_eq!(0x0101, cpu.registers().get16(Reg16::HL));

    run(&[0xed, 0x76], &mut cpu, &mut machine); // SLP
    assert!(cpu.is_halted());
}

#[test]
fn gameboy_mode_uses_lr35902_opcodes_and_flags() {
    let mut cpu = Cpu::new_gameboy();
    let mut machine = PlainMachine::new();

    // Game Boy-only immediate high-memory load/store instructions.
    cpu.registers().set_a(0x5a);
    run(&[0xe0, 0x80], &mut cpu, &mut machine); // LDH ($80),A
    assert_eq!(0x5a, machine.peek(0xff80));
    cpu.registers().set_a(0);
    run(&[0xf0, 0x80], &mut cpu, &mut machine); // LDH A,($80)
    assert_eq!(0x5a, cpu.registers().a());

    // The LR35902 F register only retains Z/N/H/C, unlike a Z80 F register.
    cpu.registers().set_a(0x0f);
    run(&[0xc6, 0x01], &mut cpu, &mut machine); // ADD A,$01
    assert_eq!(0x10, cpu.registers().a());
    assert_eq!(0x20, cpu.registers().get8(Reg8::F));

    // Signed SP arithmetic uses the low-byte and low-nibble carry rules.
    cpu.registers().set16(Reg16::SP, 0x00ff);
    run(&[0xf8, 0x01], &mut cpu, &mut machine); // LD HL,SP+$01
    assert_eq!(0x0100, cpu.registers().get16(Reg16::HL));
    assert_eq!(0x30, cpu.registers().get8(Reg8::F));

    // CB $30 is Game Boy SWAP, not a Z80 shift opcode.
    cpu.registers().set8(Reg8::B, 0xf0);
    run(&[0xcb, 0x30], &mut cpu, &mut machine);
    assert_eq!(0x0f, cpu.registers().get8(Reg8::B));
    assert_eq!(0x00, cpu.registers().get8(Reg8::F));

    // A Z80-only DD prefix is an illegal LR35902 instruction.
    run(&[0xdd], &mut cpu, &mut machine);
    assert!(cpu.state.illegal_instruction);
}

#[test]
fn ez80_mode_supports_adl_and_size_prefixes() {
    let mut cpu = Cpu::new_ez80();
    let mut machine = PlainMachine::new();
    cpu.set_adl(true);
    run(&[0x01, 0x34, 0x12, 0xab], &mut cpu, &mut machine); // LD BC,$ab1234
    assert_eq!(0xab1234, cpu.registers().get24(Reg16::BC));

    cpu.registers().set24(Reg16::BC, 0);
    run(&[0x40, 0x01, 0x78, 0x56], &mut cpu, &mut machine); // LD.SIS BC,$5678
    assert_eq!(0x005678, cpu.registers().get24(Reg16::BC));
}
