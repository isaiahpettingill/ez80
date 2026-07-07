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

    for mode in [
        CpuMode::I8080,
        CpuMode::I8085,
        CpuMode::Z80,
        CpuMode::Z80N,
        CpuMode::Z180,
        CpuMode::EZ80,
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
    run(&[0xed, 0x91, 0x12], &mut cpu, &mut machine); // NEXTREG $12,A
    assert_eq!(0x12, machine.port_in(0x243b));
    assert_eq!(0x99, machine.port_in(0x253b));

    run(&[0xed, 0x92, 0x34, 0x56], &mut cpu, &mut machine); // NEXTREG $34,$56
    assert_eq!(0x34, machine.port_in(0x243b));
    assert_eq!(0x56, machine.port_in(0x253b));
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
