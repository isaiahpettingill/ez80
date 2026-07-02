use ez80::{Cpu, FastBus, Flag, Machine, Reg16, Reg8};
use std::cell::Cell;

const MEM_SIZE: usize = 4 * 65536;
const IO_SIZE: usize = 65536;

#[derive(Debug, Clone, PartialEq, Eq)]
enum IoEvent {
    Input(u16, u8),
    Output(u16, u8),
}

struct TraceBus {
    mem: Box<[u8; MEM_SIZE]>,
    io: Box<[u8; IO_SIZE]>,
    cycles: Cell<i64>,
    writes: Vec<(u32, u8)>,
    io_events: Vec<IoEvent>,
}

impl TraceBus {
    fn new(program: &[u8]) -> Self {
        let mut bus = Self {
            mem: Box::new([0; MEM_SIZE]),
            io: Box::new([0; IO_SIZE]),
            cycles: Cell::new(0),
            writes: Vec::new(),
            io_events: Vec::new(),
        };
        bus.mem[..program.len()].copy_from_slice(program);
        bus
    }
}

impl Machine for TraceBus {
    fn peek(&self, address: u32) -> u8 {
        self.use_cycles(1);
        self.mem[address as usize]
    }

    fn poke(&mut self, address: u32, value: u8) {
        self.use_cycles(1);
        self.mem[address as usize] = value;
        self.writes.push((address, value));
    }

    fn use_cycles(&self, cycles: i32) {
        self.cycles
            .set(self.cycles.get().wrapping_add(cycles as i64));
    }

    fn port_in(&mut self, address: u16) -> u8 {
        self.use_cycles(1);
        let value = self.io[address as usize];
        self.io_events.push(IoEvent::Input(address, value));
        value
    }

    fn port_out(&mut self, address: u16, value: u8) {
        self.use_cycles(1);
        self.io[address as usize] = value;
        self.io_events.push(IoEvent::Output(address, value));
    }
}

impl FastBus for TraceBus {
    fn read8(&mut self, addr: u32) -> u8 {
        self.mem[addr as usize]
    }

    fn write8(&mut self, addr: u32, value: u8) {
        self.mem[addr as usize] = value;
        self.writes.push((addr, value));
    }

    fn input8(&mut self, port: u16) -> u8 {
        let value = self.io[port as usize];
        self.io_events.push(IoEvent::Input(port, value));
        value
    }

    fn output8(&mut self, port: u16, value: u8) {
        self.io[port as usize] = value;
        self.io_events.push(IoEvent::Output(port, value));
    }

    fn add_cycles(&mut self, cycles: u32) {
        self.cycles
            .set(self.cycles.get().wrapping_add(cycles as i64));
    }
}

fn compare_cpu(reference: &mut Cpu, fast: &mut Cpu) {
    assert_eq!(reference.state.pc(), fast.state.pc(), "PC");
    assert_eq!(reference.state.halted, fast.state.halted, "halted");
    assert_eq!(
        reference.state.nmi_pending, fast.state.nmi_pending,
        "nmi_pending"
    );
    assert_eq!(
        reference.state.reset_pending, fast.state.reset_pending,
        "reset_pending"
    );
    assert_eq!(reference.state.reg.adl, fast.state.reg.adl, "ADL");
    assert_eq!(reference.state.reg.madl, fast.state.reg.madl, "MADL");
    assert_eq!(reference.state.reg.mbase, fast.state.reg.mbase, "MBASE");
    assert_eq!(reference.state.reg.iff1, fast.state.reg.iff1, "IFF1");
    assert_eq!(reference.state.reg.iff2, fast.state.reg.iff2, "IFF2");
    assert_eq!(
        reference.state.illegal_instruction, fast.state.illegal_instruction,
        "illegal"
    );
    assert_eq!(
        reference.state.cached_instruction, fast.state.cached_instruction,
        "cached"
    );
    assert_eq!(
        reference.state.instructions_executed, fast.state.instructions_executed,
        "insns"
    );

    for reg in [
        Reg8::A,
        Reg8::F,
        Reg8::B,
        Reg8::C,
        Reg8::D,
        Reg8::E,
        Reg8::H,
        Reg8::L,
        Reg8::I,
        Reg8::R,
    ] {
        assert_eq!(
            reference.registers().get8(reg),
            fast.registers().get8(reg),
            "{:?}",
            reg
        );
    }
    for reg in [
        Reg16::AF,
        Reg16::BC,
        Reg16::DE,
        Reg16::HL,
        Reg16::IX,
        Reg16::IY,
        Reg16::SP,
    ] {
        assert_eq!(
            reference.registers().get16(reg),
            fast.registers().get16(reg),
            "{:?} 16",
            reg
        );
        if reg != Reg16::AF {
            assert_eq!(
                reference.registers().get24(reg),
                fast.registers().get24(reg),
                "{:?} 24",
                reg
            );
        }
    }
    for flag in [
        Flag::C,
        Flag::N,
        Flag::P,
        Flag::_3,
        Flag::H,
        Flag::_5,
        Flag::Z,
        Flag::S,
    ] {
        assert_eq!(
            reference.registers().get_flag(flag),
            fast.registers().get_flag(flag),
            "{:?}",
            flag
        );
    }
}

fn run_compare(program: &[u8], steps: usize, configure: fn(&mut Cpu, &mut TraceBus)) {
    let mut reference = Cpu::new_ez80();
    let mut reference_bus = TraceBus::new(program);
    configure(&mut reference, &mut reference_bus);

    let mut fast = Cpu::new_ez80();
    let mut fast_bus = TraceBus::new(program);
    configure(&mut fast, &mut fast_bus);

    for _ in 0..steps {
        reference.execute_instruction(&mut reference_bus);
        fast.step_fast(&mut fast_bus);
    }

    compare_cpu(&mut reference, &mut fast);
    assert_eq!(
        reference_bus.cycles.get() as u64,
        fast.cycles(),
        "cycle count"
    );
    assert_eq!(reference_bus.writes, fast_bus.writes, "memory writes");
    assert_eq!(
        reference_bus.io_events, fast_bus.io_events,
        "port I/O order"
    );
}

fn default_config(_: &mut Cpu, _: &mut TraceBus) {}

#[test]
fn fast_path_matches_reference_register_alu() {
    run_compare(
        &[0x3e, 0x12, 0x06, 0x34, 0x80, 0xa8, 0x3c, 0x15],
        6,
        default_config,
    );
}

#[test]
fn fast_path_matches_reference_memory_io_and_branch() {
    fn configure(cpu: &mut Cpu, bus: &mut TraceBus) {
        cpu.registers().set16(Reg16::HL, 0x2000);
        cpu.registers().set16(Reg16::DE, 0x2100);
        cpu.registers().set8(Reg8::A, 0x12);
        bus.mem[0x2000] = 0xa5;
        bus.io[0x12fe] = 0x7b;
    }
    run_compare(
        &[0x7e, 0x12, 0xdb, 0xfe, 0xd3, 0x80, 0x18, 0xfe],
        6,
        configure,
    );
}

#[test]
fn fast_path_matches_reference_adl_indexed_and_size_prefixes() {
    fn configure(cpu: &mut Cpu, bus: &mut TraceBus) {
        cpu.set_adl(true);
        cpu.registers().set24(Reg16::IX, 0x2000);
        cpu.registers().set24(Reg16::IY, 0x2100);
        bus.mem[0x2001] = 0x9c;
        bus.mem[0x20000] = 0x5a;
    }
    run_compare(
        &[
            0xdd, 0x7e, 0x01, 0xfd, 0x77, 0x01, 0x3a, 0x00, 0x00, 0x02, 0x32, 0x02, 0x00, 0x02,
        ],
        4,
        configure,
    );
}

#[test]
fn fast_path_save_load_and_run_cycles_are_deterministic() {
    let mut cpu = Cpu::new_ez80();
    let mut bus = TraceBus::new(&[0x00, 0x00, 0x00, 0x00]);
    cpu.step_fast(&mut bus);
    let saved = cpu.save_state();
    let saved_cycles = cpu.cycles();
    cpu.run_cycles(&mut bus, 2);
    cpu.load_state(&saved);
    cpu.set_cycles(saved_cycles);
    cpu.run_until(&mut bus, saved_cycles + 2);
    assert_eq!(cpu.state.pc(), 3);
    assert_eq!(cpu.cycles(), saved_cycles + 2);
}
