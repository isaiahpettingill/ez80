use ez80::{Cpu, FastBus, Machine, Reg16};
use std::cell::Cell;
use std::hint::black_box;
use std::time::{Duration, Instant};

const MEM_SIZE: usize = 4 * 65536;
const IO_SIZE: usize = 65536;
const INSTRUCTIONS: usize = 250_000;

struct BenchBus {
    mem: Box<[u8; MEM_SIZE]>,
    io: Box<[u8; IO_SIZE]>,
    cycles: Cell<i64>,
}

impl BenchBus {
    fn new() -> Self {
        Self {
            mem: Box::new([0; MEM_SIZE]),
            io: Box::new([0; IO_SIZE]),
            cycles: Cell::new(0),
        }
    }

    fn cycles(&self) -> u64 {
        self.cycles.get() as u64
    }

    fn load_repeated(&mut self, pattern: &[u8], instructions: usize) {
        for i in 0..instructions {
            let start = i * pattern.len();
            self.mem[start..start + pattern.len()].copy_from_slice(pattern);
        }
    }
}

impl Machine for BenchBus {
    fn peek(&self, address: u32) -> u8 {
        self.use_cycles(1);
        self.mem[address as usize]
    }

    fn poke(&mut self, address: u32, value: u8) {
        self.use_cycles(1);
        self.mem[address as usize] = value;
    }

    fn use_cycles(&self, cycles: i32) {
        self.cycles
            .set(self.cycles.get().wrapping_add(cycles as i64));
    }

    fn port_in(&mut self, address: u16) -> u8 {
        self.use_cycles(1);
        self.io[address as usize]
    }

    fn port_out(&mut self, address: u16, value: u8) {
        self.use_cycles(1);
        self.io[address as usize] = value;
    }
}

impl FastBus for BenchBus {
    fn read8(&mut self, addr: u32) -> u8 {
        self.mem[addr as usize]
    }

    fn write8(&mut self, addr: u32, value: u8) {
        self.mem[addr as usize] = value;
    }

    fn input8(&mut self, port: u16) -> u8 {
        self.io[port as usize]
    }

    fn output8(&mut self, port: u16, value: u8) {
        self.io[port as usize] = value;
    }

    fn add_cycles(&mut self, cycles: u32) {
        self.cycles
            .set(self.cycles.get().wrapping_add(cycles as i64));
    }
}

#[derive(Clone, Copy)]
struct Measurement {
    instructions: usize,
    cycles: u64,
    elapsed: Duration,
}

impl Measurement {
    fn ips(self) -> f64 {
        self.instructions as f64 / self.elapsed.as_secs_f64()
    }

    fn cps(self) -> f64 {
        self.cycles as f64 / self.elapsed.as_secs_f64()
    }
}

fn measure_reference(mut cpu: Cpu, mut bus: BenchBus, instructions: usize) -> Measurement {
    let start = Instant::now();
    for _ in 0..instructions {
        cpu.execute_instruction(&mut bus);
    }
    let elapsed = start.elapsed();
    black_box(cpu.state.pc());
    black_box(bus.mem[0]);
    Measurement {
        instructions,
        cycles: bus.cycles(),
        elapsed,
    }
}

fn measure_fast(mut cpu: Cpu, mut bus: BenchBus, instructions: usize) -> Measurement {
    let start = Instant::now();
    for _ in 0..instructions {
        cpu.step_fast(&mut bus);
    }
    let elapsed = start.elapsed();
    black_box(cpu.state.pc());
    black_box(bus.mem[0]);
    Measurement {
        instructions,
        cycles: cpu.cycles(),
        elapsed,
    }
}

fn print_row(name: &str, reference: Measurement, fast: Measurement) {
    println!(
        "{:<24} {:>12.0} {:>12.0} {:>12.0} {:>12.0} {:>8.2}x",
        name,
        reference.ips(),
        fast.ips(),
        reference.cps(),
        fast.cps(),
        fast.ips() / reference.ips()
    );
}

fn bench_linear(name: &str, configure: fn(&mut Cpu, &mut BenchBus), pattern: &[u8]) {
    let instruction_count = INSTRUCTIONS.min(MEM_SIZE / pattern.len());

    let mut reference_bus = BenchBus::new();
    reference_bus.load_repeated(pattern, instruction_count);
    let mut reference_cpu = Cpu::new_ez80();
    configure(&mut reference_cpu, &mut reference_bus);

    let mut fast_bus = BenchBus::new();
    fast_bus.load_repeated(pattern, instruction_count);
    let mut fast_cpu = Cpu::new_ez80();
    configure(&mut fast_cpu, &mut fast_bus);

    let reference = measure_reference(reference_cpu, reference_bus, instruction_count);
    let fast = measure_fast(fast_cpu, fast_bus, instruction_count);
    print_row(name, reference, fast);
}

fn bench_branch_loop() {
    let mut reference_bus = BenchBus::new();
    reference_bus.mem[0] = 0x18;
    reference_bus.mem[1] = 0xfe;
    let reference_cpu = Cpu::new_ez80();

    let mut fast_bus = BenchBus::new();
    fast_bus.mem[0] = 0x18;
    fast_bus.mem[1] = 0xfe;
    let fast_cpu = Cpu::new_ez80();

    let reference = measure_reference(reference_cpu, reference_bus, INSTRUCTIONS);
    let fast = measure_fast(fast_cpu, fast_bus, INSTRUCTIONS);
    print_row("branch loop", reference, fast);
}

fn bench_run_cycles() {
    let mut bus = BenchBus::new();
    for byte in bus.mem.iter_mut() {
        *byte = 0x00;
    }
    let mut cpu = Cpu::new_ez80();
    let start = Instant::now();
    cpu.run_cycles(&mut bus, INSTRUCTIONS as u64);
    let elapsed = start.elapsed();
    let measurement = Measurement {
        instructions: cpu.state.instructions_executed as usize,
        cycles: cpu.cycles(),
        elapsed,
    };
    println!(
        "{:<24} {:>12} {:>12.0} {:>12} {:>12.0} {:>8}",
        "run_cycles loop",
        "-",
        measurement.ips(),
        "-",
        measurement.cps(),
        "-"
    );
}

fn bench_save_load() {
    let mut cpu = Cpu::new_ez80();
    cpu.set_adl(true);
    cpu.registers().set24(Reg16::BC, 0x123456);
    cpu.registers().set24(Reg16::DE, 0x234567);
    cpu.registers().set24(Reg16::HL, 0x345678);

    let saved = cpu.save_state();
    let iterations = 1_000_000;
    let start = Instant::now();
    for _ in 0..iterations {
        let state = black_box(cpu.save_state());
        cpu.load_state(black_box(&saved));
        black_box(state);
    }
    let elapsed = start.elapsed();
    let ns_per_pair = elapsed.as_nanos() as f64 / iterations as f64;
    println!(
        "{:<24} {:>12} {:>12} {:>12} {:>12} {:>7.1}ns",
        "save/load state", "-", "-", "-", "-", ns_per_pair
    );
}

fn default_config(_: &mut Cpu, _: &mut BenchBus) {}

fn memory_config(cpu: &mut Cpu, bus: &mut BenchBus) {
    cpu.registers().set16(Reg16::HL, 0x2000);
    cpu.registers().set16(Reg16::DE, 0x2100);
    bus.mem[0x2000] = 0xa5;
}

fn adl_config(cpu: &mut Cpu, bus: &mut BenchBus) {
    cpu.set_adl(true);
    bus.mem[0x20000] = 0x5a;
}

fn indexed_config(cpu: &mut Cpu, bus: &mut BenchBus) {
    cpu.set_adl(true);
    cpu.registers().set24(Reg16::IX, 0x2000);
    cpu.registers().set24(Reg16::IY, 0x2100);
    bus.mem[0x2001] = 0x9c;
}

fn main() {
    println!(
        "{:<24} {:>12} {:>12} {:>12} {:>12} {:>9}",
        "benchmark", "ref ips", "fast ips", "ref cps", "fast cps", "speedup"
    );
    println!("{}", "-".repeat(89));

    bench_linear("NOP loop", default_config, &[0x00]);
    bench_linear(
        "register ALU loop",
        default_config,
        &[0x3c, 0x80, 0xa9, 0x15],
    );
    bench_linear("memory read/write", memory_config, &[0x7e, 0x12]);
    bench_branch_loop();
    bench_linear(
        "ADL load/store",
        adl_config,
        &[0x3a, 0x00, 0x00, 0x02, 0x32, 0x01, 0x00, 0x02],
    );
    bench_linear(
        "IX/IY indexed",
        indexed_config,
        &[0xdd, 0x7e, 0x01, 0xfd, 0x77, 0x01],
    );
    bench_save_load();
    bench_run_cycles();
}
