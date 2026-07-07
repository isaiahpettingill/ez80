use ez80::{Cpu, Machine, Reg16, Reg8};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

const MEM_SIZE: usize = 16 * 1024 * 1024;
const DEFAULT_LOAD_ADDR: u32 = 0x040000;
const DEFAULT_STACK: u32 = 0x0ffff0;

const DEMO_PROGRAM: &[u8] = &[
    0x01, 0x0e, 0x00, 0x04, // ld bc,$04000e
    0x0a, // loop: ld a,(bc)
    0xb7, // or a
    0x28, 0x05, // jr z,done
    0xd3, 0x00, // out ($00),a
    0x03, // inc bc
    0x18, 0xf7, // jr loop
    0x76, // done: halt
    b'A', b'g', b'o', b'n', b' ', b'L', b'i', b'g', b'h', b't', b' ', b'h', b'e', b'a', b'd', b'l',
    b'e', b's', b's', b' ', b'd', b'e', b'm', b'o', b'\n', 0,
];

#[derive(Default)]
struct Args {
    program: Option<PathBuf>,
    load_addr: u32,
    pc: Option<u32>,
    stack: u32,
    max_steps: u64,
    trace_io: bool,
}

struct AgonLightHeadless {
    mem: Vec<u8>,
    input: [u8; 65536],
    trace_io: bool,
}

impl AgonLightHeadless {
    fn new(trace_io: bool) -> Self {
        let mut input = [0xff; 65536];
        input[0x00] = 0x00;
        input[0x05] = 0x60;
        Self {
            mem: vec![0; MEM_SIZE],
            input,
            trace_io,
        }
    }

    fn load(&mut self, addr: u32, code: &[u8]) {
        let start = addr as usize;
        let end = start
            .checked_add(code.len())
            .expect("program address overflow");
        if end > self.mem.len() {
            panic!("program does not fit in 24-bit Agon memory");
        }
        self.mem[start..end].copy_from_slice(code);
    }

    fn read_c_string(&self, addr: u32) -> String {
        let mut bytes = Vec::new();
        for offset in 0..4096 {
            let byte = self.peek(addr.wrapping_add(offset));
            if byte == 0 {
                break;
            }
            bytes.push(byte);
        }
        String::from_utf8_lossy(&bytes).into_owned()
    }

    fn print_byte(&self, value: u8) {
        print!("{}", value as char);
        io::stdout().flush().expect("flush stdout");
    }
}

impl Machine for AgonLightHeadless {
    fn peek(&self, address: u32) -> u8 {
        self.mem[address as usize % self.mem.len()]
    }

    fn poke(&mut self, address: u32, value: u8) {
        let index = address as usize % self.mem.len();
        self.mem[index] = value;
    }

    fn use_cycles(&self, _cycles: i32) {}

    fn port_in(&mut self, address: u16) -> u8 {
        let value = self.input[address as usize];
        if self.trace_io {
            eprintln!("in  ${address:04x} -> ${value:02x}");
        }
        value
    }

    fn port_out(&mut self, address: u16, value: u8) {
        if self.trace_io {
            eprintln!("out ${address:04x} <- ${value:02x}");
        }
        match address & 0x00ff {
            0x00 | 0x01 | 0x02 => self.print_byte(value),
            _ => {}
        }
    }
}

fn main() {
    let args = parse_args();
    let (program, name) = match args.program {
        Some(path) => (
            fs::read(&path).expect("read program"),
            path.display().to_string(),
        ),
        None => (DEMO_PROGRAM.to_vec(), "built-in demo".to_string()),
    };

    let mut machine = AgonLightHeadless::new(args.trace_io);
    machine.load(args.load_addr, &program);

    let mut cpu = Cpu::new_ez80();
    cpu.set_adl(true);
    cpu.state.set_pc(args.pc.unwrap_or(args.load_addr));
    cpu.registers().set24(Reg16::SP, args.stack);

    eprintln!(
        "loaded {name}: {} bytes at ${:06x}, pc=${:06x}",
        program.len(),
        args.load_addr,
        cpu.state.pc()
    );

    let stop = run(&mut cpu, &mut machine, args.max_steps);
    eprintln!(
        "\nstopped: {stop}; pc=${:06x}; steps={}",
        cpu.state.pc(),
        cpu.state.instructions_executed
    );
}

fn run(cpu: &mut Cpu, machine: &mut AgonLightHeadless, max_steps: u64) -> String {
    for _ in 0..max_steps {
        if cpu.is_halted() {
            return "halted".to_string();
        }
        if let Some(stop) = handle_semihosted_rst(cpu, machine) {
            return stop;
        }
        cpu.execute_instruction(machine);
        if cpu.state.illegal_instruction || cpu.state.illegal_instruction_adl {
            return "illegal instruction".to_string();
        }
    }
    format!("max steps reached ({max_steps})")
}

fn handle_semihosted_rst(cpu: &mut Cpu, machine: &mut AgonLightHeadless) -> Option<String> {
    let pc = cpu.state.pc();
    let first = machine.peek(pc);
    let second = machine.peek(pc.wrapping_add(1));
    let is_lil_rst_08 = first == 0x5b && second == 0xcf;
    let is_lil_rst_10 = first == 0x5b && second == 0xd7;
    let is_rst_08 = first == 0xcf;
    let is_rst_10 = first == 0xd7;

    if !(is_lil_rst_08 || is_lil_rst_10 || is_rst_08 || is_rst_10) {
        return None;
    }

    cpu.state
        .set_pc(pc.wrapping_add(if is_lil_rst_08 || is_lil_rst_10 { 2 } else { 1 }));

    if is_lil_rst_10 || is_rst_10 {
        let value = cpu.registers().get8(Reg8::A);
        machine.print_byte(value);
        return None;
    }

    match cpu.registers().get8(Reg8::A) {
        0x00 => Some("program requested exit through RST 08h/A=0".to_string()),
        0x01 => {
            let value = cpu.registers().get8(Reg8::C);
            machine.print_byte(value);
            None
        }
        0x02 => {
            let addr = cpu.registers().get24(Reg16::HL);
            print!("{}", machine.read_c_string(addr));
            io::stdout().flush().expect("flush stdout");
            None
        }
        function => Some(format!("unsupported semihosted RST 08h/A=${function:02x}")),
    }
}

fn parse_args() -> Args {
    let mut args = Args {
        load_addr: DEFAULT_LOAD_ADDR,
        stack: DEFAULT_STACK,
        max_steps: 2_000_000,
        ..Args::default()
    };
    let mut iter = env::args().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--program" => args.program = Some(PathBuf::from(next_value(&mut iter, "--program"))),
            "--addr" => args.load_addr = parse_u32(&next_value(&mut iter, "--addr")),
            "--pc" => args.pc = Some(parse_u32(&next_value(&mut iter, "--pc"))),
            "--stack" => args.stack = parse_u32(&next_value(&mut iter, "--stack")),
            "--max-steps" => args.max_steps = parse_u64(&next_value(&mut iter, "--max-steps")),
            "--trace-io" => args.trace_io = true,
            "--help" | "-h" => usage(),
            _ => usage(),
        }
    }
    args
}

fn next_value(iter: &mut impl Iterator<Item = String>, flag: &str) -> String {
    iter.next()
        .unwrap_or_else(|| panic!("missing value for {}", flag))
}

fn parse_u32(value: &str) -> u32 {
    let trimmed = value.trim_start_matches("0x").trim_start_matches('$');
    u32::from_str_radix(trimmed, 16).unwrap_or_else(|_| panic!("bad hex value: {}", value))
}

fn parse_u64(value: &str) -> u64 {
    value
        .parse()
        .unwrap_or_else(|_| panic!("bad integer: {}", value))
}

fn usage() -> ! {
    eprintln!(
        "usage: cargo run --example agon_light_headless -- [--program file.bin] [--addr 040000] [--pc 040000] [--max-steps n] [--trace-io]"
    );
    std::process::exit(2);
}
