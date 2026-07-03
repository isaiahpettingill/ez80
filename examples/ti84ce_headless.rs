use ez80::{Cpu, FastBus, Machine, Reg16};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::env;
use std::fs;
use std::path::PathBuf;

const MEM_SIZE: usize = 16 * 1024 * 1024;
const KEY_PORT: u16 = 0x0001;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Key {
    group: u8,
    bit: u8,
}

#[derive(Default)]
struct Args {
    rom: Option<PathBuf>,
    script: String,
    script_file: Option<PathBuf>,
    max_steps: u64,
    boot_friendly: bool,
}

struct Ti84CeHeadless {
    mem: Vec<u8>,
    input: [u8; 65536],
    output: [u8; 65536],
    keypad_select: u8,
    keys_down: BTreeSet<Key>,
    cycles: u64,
    port_in: BTreeMap<u16, u64>,
    port_out: BTreeMap<u16, u64>,
}

impl Ti84CeHeadless {
    fn new(rom: &[u8], boot_friendly: bool) -> Self {
        let mut mem = vec![0xff; MEM_SIZE];
        mem[..rom.len().min(MEM_SIZE)].copy_from_slice(&rom[..rom.len().min(MEM_SIZE)]);

        let mut input = [0xff; 65536];
        if boot_friendly {
            // These let common TI-OS boot polling loops make progress in a
            // headless scaffold. Replace them with real ASIC models as needed.
            input[0x000d] = 0xff;
            input[0xd00c] = 0x00;
            input[0xd00d] = 0x00;
        }

        Self {
            mem,
            input,
            output: [0; 65536],
            keypad_select: 0xff,
            keys_down: BTreeSet::new(),
            cycles: 0,
            port_in: BTreeMap::new(),
            port_out: BTreeMap::new(),
        }
    }

    fn read_mem(&self, addr: u32) -> u8 {
        self.mem[addr as usize % self.mem.len()]
    }

    fn write_mem(&mut self, addr: u32, value: u8) {
        let index = addr as usize % self.mem.len();
        self.mem[index] = value;
    }

    fn input_port(&mut self, port: u16) -> u8 {
        *self.port_in.entry(port).or_insert(0) += 1;
        if port == KEY_PORT {
            return self.read_keypad();
        }
        self.input[port as usize]
    }

    fn output_port(&mut self, port: u16, value: u8) {
        *self.port_out.entry(port).or_insert(0) += 1;
        self.output[port as usize] = value;
        if port == KEY_PORT {
            self.keypad_select = value;
        }
    }

    fn read_keypad(&self) -> u8 {
        let mut value = 0xff;
        if self.keypad_select == 0xff {
            return value;
        }
        for key in &self.keys_down {
            if (self.keypad_select & (1 << key.group)) == 0 {
                value &= !(1 << key.bit);
            }
        }
        value
    }

    fn press(&mut self, key: Key) {
        self.keys_down.insert(key);
    }

    fn release(&mut self, key: Key) {
        self.keys_down.remove(&key);
    }

    fn set_port(&mut self, port: u16, value: u8) {
        self.input[port as usize] = value;
    }
}

impl Machine for Ti84CeHeadless {
    fn peek(&self, address: u32) -> u8 {
        self.read_mem(address)
    }

    fn poke(&mut self, address: u32, value: u8) {
        self.write_mem(address, value);
    }

    fn use_cycles(&self, _cycles: i32) {}

    fn port_in(&mut self, address: u16) -> u8 {
        self.input_port(address)
    }

    fn port_out(&mut self, address: u16, value: u8) {
        self.output_port(address, value);
    }
}

impl FastBus for Ti84CeHeadless {
    fn read8(&mut self, addr: u32) -> u8 {
        self.read_mem(addr)
    }

    fn write8(&mut self, addr: u32, value: u8) {
        self.write_mem(addr, value);
    }

    fn input8(&mut self, port: u16) -> u8 {
        self.input_port(port)
    }

    fn output8(&mut self, port: u16, value: u8) {
        self.output_port(port, value);
    }

    fn add_cycles(&mut self, cycles: u32) {
        self.cycles = self.cycles.wrapping_add(cycles as u64);
    }
}

fn main() {
    let args = parse_args();
    let rom_path = args.rom.unwrap_or_else(|| usage("missing --rom <path>"));
    let mut script = args.script;
    if let Some(path) = args.script_file {
        script.push(';');
        script.push_str(&fs::read_to_string(path).expect("read script file"));
    }

    let rom = fs::read(&rom_path).expect("read ROM");
    let mut cpu = Cpu::new_ez80();
    let mut hw = Ti84CeHeadless::new(&rom, args.boot_friendly);
    println!(
        "loaded ROM: {} bytes from {}",
        rom.len(),
        rom_path.display()
    );

    if script.trim().is_empty() {
        run(&mut cpu, &mut hw, args.max_steps, false);
    } else {
        run_script(&mut cpu, &mut hw, &script, args.max_steps);
    }

    print_summary(&cpu, &hw);
}

fn parse_args() -> Args {
    let mut args = Args {
        max_steps: 2_000_000,
        boot_friendly: true,
        ..Args::default()
    };
    let mut iter = env::args().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--rom" => args.rom = Some(PathBuf::from(next_value(&mut iter, "--rom"))),
            "--script" => args.script = next_value(&mut iter, "--script"),
            "--script-file" => {
                args.script_file = Some(PathBuf::from(next_value(&mut iter, "--script-file")))
            }
            "--max-steps" => args.max_steps = parse_u64(&next_value(&mut iter, "--max-steps")),
            "--no-boot-friendly" => args.boot_friendly = false,
            "--help" | "-h" => usage(""),
            _ => usage(&format!("unknown argument: {arg}")),
        }
    }
    args
}

fn next_value(iter: &mut impl Iterator<Item = String>, flag: &str) -> String {
    iter.next()
        .unwrap_or_else(|| usage(&format!("missing value for {flag}")))
}

fn run_script(cpu: &mut Cpu, hw: &mut Ti84CeHeadless, script: &str, default_steps: u64) {
    for raw in script.split([';', '\n']) {
        let command = raw.trim();
        if command.is_empty() || command.starts_with('#') {
            continue;
        }
        let parts = command.split(':').collect::<Vec<_>>();
        match parts.as_slice() {
            ["wait", steps] | ["step", steps] => {
                run(cpu, hw, parse_u64(steps), false);
            }
            ["until-wait", steps] => run(cpu, hw, parse_u64(steps), true),
            ["press", keys] => {
                for_each_key(keys, |key| hw.press(key));
                cpu.state.halted = false;
            }
            ["release", keys] => for_each_key(keys, |key| hw.release(key)),
            ["tap", keys] => tap(cpu, hw, keys, 20),
            ["tap", keys, steps] => tap(cpu, hw, keys, parse_u64(steps)),
            ["setport", assignment] => {
                let (port, value) = parse_assignment(assignment);
                hw.set_port(port, value);
            }
            ["dump"] => print_summary(cpu, hw),
            ["run"] => run(cpu, hw, default_steps, false),
            _ => usage(&format!("bad script command: {command}")),
        }
    }
}

fn tap(cpu: &mut Cpu, hw: &mut Ti84CeHeadless, keys: &str, steps: u64) {
    let parsed = parse_keys(keys);
    for key in &parsed {
        hw.press(*key);
    }
    cpu.state.halted = false;
    run(cpu, hw, steps, false);
    for key in &parsed {
        hw.release(*key);
    }
    run(cpu, hw, steps, false);
}

fn run(cpu: &mut Cpu, hw: &mut Ti84CeHeadless, max_steps: u64, stop_on_wait: bool) {
    let mut recent = VecDeque::new();
    for step in 0..max_steps {
        let pc = cpu.state.pc();
        recent.push_back(pc);
        if recent.len() > 32 {
            recent.pop_front();
        }
        cpu.step_fast(hw);
        if cpu.state.illegal_instruction {
            println!("illegal instruction at pc=${pc:06x}");
            return;
        }
        if cpu.is_halted() {
            println!(
                "halted after {} steps at pc=${:06x}",
                step + 1,
                cpu.state.pc()
            );
            return;
        }
        if stop_on_wait && cpu.state.instructions_executed > 10_000 && is_tight_wait(&recent) {
            println!(
                "stopped at likely hardware wait loop pc=${:06x}",
                cpu.state.pc()
            );
            return;
        }
    }
}

fn is_tight_wait(recent: &VecDeque<u32>) -> bool {
    if recent.len() < 32 {
        return false;
    }
    recent.iter().copied().collect::<BTreeSet<_>>().len() <= 4
}

fn print_summary(cpu: &Cpu, hw: &Ti84CeHeadless) {
    println!(
        "pc=${:06x} adl={} mb=${:02x} af=${:04x} bc=${:06x} de=${:06x} hl=${:06x} ix=${:06x} iy=${:06x} sp=${:06x} instr={} cycles={}",
        cpu.state.pc(),
        cpu.state.reg.adl,
        cpu.state.reg.mbase,
        cpu.state.reg.get16(Reg16::AF),
        cpu.state.reg.get24(Reg16::BC),
        cpu.state.reg.get24(Reg16::DE),
        cpu.state.reg.get24(Reg16::HL),
        cpu.state.reg.get24(Reg16::IX),
        cpu.state.reg.get24(Reg16::IY),
        cpu.state.reg.get24(Reg16::SP),
        cpu.state.instructions_executed,
        hw.cycles,
    );
    println!("keys_down={:?}", hw.keys_down);
    println!("port_in_top={:?}", top_ports(&hw.port_in));
    println!("port_out_top={:?}", top_ports(&hw.port_out));
}

fn top_ports(map: &BTreeMap<u16, u64>) -> Vec<(u16, u64)> {
    let mut items = map
        .iter()
        .map(|(port, count)| (*port, *count))
        .collect::<Vec<_>>();
    items.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
    items.truncate(12);
    items
}

fn for_each_key(keys: &str, mut f: impl FnMut(Key)) {
    for key in parse_keys(keys) {
        f(key);
    }
}

fn parse_keys(keys: &str) -> Vec<Key> {
    if keys.trim() == "+" {
        return vec![parse_key("plus")];
    }
    keys.split('+').map(parse_key).collect()
}

fn parse_key(name: &str) -> Key {
    match normalize_key(name).as_str() {
        "down" => key(0, 0),
        "left" => key(0, 1),
        "right" => key(0, 2),
        "up" => key(0, 3),
        "enter" => key(1, 0),
        "+" | "plus" => key(1, 1),
        "-" | "minus" => key(2, 1),
        "*" | "mul" => key(3, 1),
        "/" | "div" => key(4, 1),
        "clear" => key(6, 0),
        "0" => key(4, 0),
        "1" => key(4, 1),
        "2" => key(3, 1),
        "3" => key(2, 1),
        "4" => key(4, 2),
        "5" => key(3, 2),
        "6" => key(2, 2),
        "7" => key(4, 3),
        "8" => key(3, 3),
        "9" => key(2, 3),
        "." | "dot" => key(3, 0),
        "neg" => key(2, 0),
        "2nd" => key(5, 5),
        "alpha" => key(7, 2),
        "mode" => key(6, 5),
        "del" => key(7, 3),
        "graph" => key(5, 0),
        "trace" => key(6, 1),
        "zoom" => key(6, 2),
        "window" => key(6, 3),
        "y=" | "y" => key(6, 4),
        "stat" => key(7, 0),
        "xttheta" | "x,t,theta,n" | "x" => key(7, 1),
        "math" => key(6, 4),
        "apps" => key(6, 3),
        "prgm" => key(6, 2),
        "vars" => key(6, 1),
        "sto" => key(5, 1),
        "ln" => key(5, 2),
        "log" => key(5, 3),
        "x2" => key(5, 4),
        "recip" | "x^-1" => key(4, 5),
        "sin" => key(3, 5),
        "cos" => key(2, 5),
        "tan" => key(1, 5),
        "^" | "pow" => key(0, 5),
        other => usage(&format!("unknown key: {other}")),
    }
}

fn key(group: u8, bit: u8) -> Key {
    Key { group, bit }
}

fn normalize_key(name: &str) -> String {
    name.trim().to_ascii_lowercase().replace('_', "")
}

fn parse_assignment(assignment: &str) -> (u16, u8) {
    let (port, value) = assignment
        .split_once('=')
        .unwrap_or_else(|| usage("setport expects port=value"));
    (parse_u16(port), parse_u8(value))
}

fn parse_u64(value: &str) -> u64 {
    value
        .replace('_', "")
        .parse()
        .unwrap_or_else(|_| usage(&format!("bad integer: {value}")))
}

fn parse_u16(value: &str) -> u16 {
    let parsed = parse_hex_or_dec(value);
    if parsed > u16::MAX as u32 {
        usage(&format!("bad u16: {value}"));
    }
    parsed as u16
}

fn parse_u8(value: &str) -> u8 {
    let parsed = parse_hex_or_dec(value);
    if parsed > u8::MAX as u32 {
        usage(&format!("bad u8: {value}"));
    }
    parsed as u8
}

fn parse_hex_or_dec(value: &str) -> u32 {
    let trimmed = value.trim().trim_start_matches('$');
    if let Some(hex) = trimmed.strip_prefix("0x") {
        u32::from_str_radix(hex, 16).unwrap_or_else(|_| usage(&format!("bad hex: {value}")))
    } else if value.starts_with('$') {
        u32::from_str_radix(trimmed, 16).unwrap_or_else(|_| usage(&format!("bad hex: {value}")))
    } else {
        trimmed
            .parse()
            .unwrap_or_else(|_| usage(&format!("bad integer: {value}")))
    }
}

fn usage(message: &str) -> ! {
    if !message.is_empty() {
        eprintln!("error: {message}\n");
    }
    eprintln!(
        "usage: cargo run --example ti84ce_headless -- --rom ROM [--script SCRIPT] [--script-file FILE] [--max-steps N]\n\
         script commands: wait:N; until-wait:N; tap:key[:N]; press:key+key; release:key+key; setport:PORT=VALUE; dump; run\n\
         example: --script \"wait:1000000; tap:clear:2000; tap:1:2000; tap:plus:2000; tap:2:2000; tap:enter:5000; dump\""
    );
    std::process::exit(if message.is_empty() { 0 } else { 2 });
}
