use ez80::{Cpu, CpuMode, Machine, Reg16, Reg8};
use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

const TPA: u16 = 0x0100;
const BDOS: u16 = 0x0005;
const DEFAULT_DMA: u16 = 0x0080;
const FCB1: u16 = 0x005c;
const FCB2: u16 = 0x006c;

fn main() -> Result<(), String> {
    let args = Args::parse()?;
    let program =
        fs::read(&args.program).map_err(|e| format!("{}: {e}", args.program.display()))?;
    if program.len() > 0xff00 {
        return Err("COM program is too large for the CP/M transient program area".to_string());
    }

    let mut machine = CpmMachine::new(args.host_dir.clone());
    machine.install_bios_and_bdos_stubs();
    machine.load(TPA, &program);
    machine.init_command_area(&args.tail);

    let mut cpu = Cpu::new_for_mode(args.mode);
    cpu.set_trace(args.trace);
    cpu.state.set_pc(TPA as u32);
    cpu.registers().set16(Reg16::SP, 0xff00);

    let mut steps = 0_u64;
    while !machine.finished && steps < args.max_steps && !cpu.is_halted() {
        cpu.execute_instruction(&mut machine);
        steps += 1;

        match cpu.state.pc() as u16 {
            0x0000 => machine.finished = true,
            BDOS => machine.handle_bdos(&mut cpu)?,
            _ => {}
        }
    }

    if steps == args.max_steps {
        return Err(format!("stopped after --max-steps {}", args.max_steps));
    }

    Ok(())
}

struct Args {
    program: PathBuf,
    tail: String,
    host_dir: PathBuf,
    mode: CpuMode,
    max_steps: u64,
    trace: bool,
}

impl Args {
    fn parse() -> Result<Self, String> {
        let mut program = None;
        let mut tail = String::new();
        let mut host_dir = env::current_dir().map_err(|e| e.to_string())?;
        let mut mode = CpuMode::Z80;
        let mut max_steps = 20_000_000;
        let mut trace = false;

        let mut args = env::args().skip(1);
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--program" => program = Some(PathBuf::from(next_arg(&mut args, "--program")?)),
                "--tail" => tail = next_arg(&mut args, "--tail")?,
                "--host-dir" => host_dir = PathBuf::from(next_arg(&mut args, "--host-dir")?),
                "--cpu" => mode = parse_mode(&next_arg(&mut args, "--cpu")?)?,
                "--max-steps" => {
                    max_steps = next_arg(&mut args, "--max-steps")?
                        .parse()
                        .map_err(|_| "--max-steps must be an integer".to_string())?
                }
                "--trace" => trace = true,
                "--help" | "-h" => return Err(usage()),
                _ if program.is_none() => program = Some(PathBuf::from(arg)),
                _ => return Err(format!("unexpected argument: {arg}\n{}", usage())),
            }
        }

        Ok(Self {
            program: program.ok_or_else(usage)?,
            tail,
            host_dir,
            mode,
            max_steps,
            trace,
        })
    }
}

fn next_arg(args: &mut impl Iterator<Item = String>, name: &str) -> Result<String, String> {
    args.next()
        .ok_or_else(|| format!("missing value for {name}\n{}", usage()))
}

fn usage() -> String {
    "usage: cargo run --example cpm -- --program PROGRAM.COM [--tail \"ARGS\"] [--host-dir DIR] [--cpu z80|8080|8085] [--max-steps N] [--trace]".to_string()
}

fn parse_mode(mode: &str) -> Result<CpuMode, String> {
    match mode.to_ascii_lowercase().as_str() {
        "z80" => Ok(CpuMode::Z80),
        "8080" => Ok(CpuMode::I8080),
        "8085" => Ok(CpuMode::I8085),
        _ => Err("--cpu must be one of: z80, 8080, 8085".to_string()),
    }
}

struct CpmMachine {
    mem: Vec<u8>,
    dma: u16,
    host_dir: PathBuf,
    finished: bool,
}

impl CpmMachine {
    fn new(host_dir: PathBuf) -> Self {
        Self {
            mem: vec![0; 0x10000],
            dma: DEFAULT_DMA,
            host_dir,
            finished: false,
        }
    }

    fn install_bios_and_bdos_stubs(&mut self) {
        self.write8(0x0000, 0xc9); // warm boot RET
        self.write8(BDOS, 0xc9); // BDOS trap returns after host handling
    }

    fn load(&mut self, address: u16, bytes: &[u8]) {
        let start = address as usize;
        self.mem[start..start + bytes.len()].copy_from_slice(bytes);
    }

    fn init_command_area(&mut self, tail: &str) {
        let tail = tail.as_bytes();
        let len = tail.len().min(126);
        self.write8(DEFAULT_DMA, len as u8);
        self.load(DEFAULT_DMA + 1, &tail[..len]);
        self.write8(DEFAULT_DMA + 1 + len as u16, b'\r');

        let mut words = String::from_utf8_lossy(tail).into_owned();
        words.make_ascii_uppercase();
        let mut words = words.split_whitespace();
        self.write_fcb(FCB1, words.next());
        self.write_fcb(FCB2, words.next());
    }

    fn handle_bdos(&mut self, cpu: &mut Cpu) -> Result<(), String> {
        match cpu.registers().get8(Reg8::C) {
            0 => self.finished = true,
            1 => self.return_byte(cpu, read_console_byte()? as u8),
            2 => write_console_byte(cpu.registers().get8(Reg8::E))?,
            6 => self.direct_console_io(cpu)?,
            9 => self.print_dollar_string(cpu.registers().get16(Reg16::DE))?,
            10 => self.read_console_buffer(cpu.registers().get16(Reg16::DE))?,
            11 => self.return_byte(cpu, 0),
            12 => cpu.registers().set16(Reg16::HL, 0x0022),
            26 => self.dma = cpu.registers().get16(Reg16::DE),
            15 => self.open_file(cpu),
            16 => self.return_byte(cpu, 0),
            20 => self.read_sequential(cpu),
            other => return Err(format!("BDOS function {other} is not implemented")),
        }
        Ok(())
    }

    fn direct_console_io(&mut self, cpu: &mut Cpu) -> Result<(), String> {
        let value = cpu.registers().get8(Reg8::E);
        if value == 0xff {
            self.return_byte(cpu, 0);
        } else {
            write_console_byte(value)?;
        }
        Ok(())
    }

    fn print_dollar_string(&self, address: u16) -> Result<(), String> {
        let mut address = address;
        while self.read8(address) != b'$' {
            write_console_byte(self.read8(address))?;
            address = address.wrapping_add(1);
        }
        Ok(())
    }

    fn read_console_buffer(&mut self, address: u16) -> Result<(), String> {
        let max = self.read8(address) as usize;
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .map_err(|e| e.to_string())?;
        let bytes = line.trim_end_matches(['\r', '\n']).as_bytes();
        let count = bytes.len().min(max);
        self.write8(address.wrapping_add(1), count as u8);
        self.load(address.wrapping_add(2), &bytes[..count]);
        write_console_byte(b'\r')?;
        write_console_byte(b'\n')?;
        Ok(())
    }

    fn open_file(&mut self, cpu: &mut Cpu) {
        let fcb = cpu.registers().get16(Reg16::DE);
        let result = self
            .fcb_host_path(fcb)
            .filter(|path| path.is_file())
            .map(|_| 0)
            .unwrap_or(0xff);
        self.write8(fcb + 32, 0);
        self.return_byte(cpu, result);
    }

    fn read_sequential(&mut self, cpu: &mut Cpu) {
        let fcb = cpu.registers().get16(Reg16::DE);
        let Some(path) = self.fcb_host_path(fcb) else {
            self.return_byte(cpu, 1);
            return;
        };
        let Ok(bytes) = fs::read(path) else {
            self.return_byte(cpu, 1);
            return;
        };

        let record = self.read8(fcb + 32) as usize;
        let offset = record * 128;
        if offset >= bytes.len() {
            self.return_byte(cpu, 1);
            return;
        }

        let count = (bytes.len() - offset).min(128);
        for i in 0..128_u16 {
            let value = if (i as usize) < count {
                bytes[offset + i as usize]
            } else {
                0x1a
            };
            self.write8(self.dma.wrapping_add(i), value);
        }
        self.write8(fcb + 32, record.wrapping_add(1) as u8);
        self.return_byte(cpu, 0);
    }

    fn write_fcb(&mut self, address: u16, name: Option<&str>) {
        self.write8(address, 0);
        for i in 1..12 {
            self.write8(address + i, b' ');
        }
        for i in 12..36 {
            self.write8(address + i, 0);
        }

        if let Some(name) = name {
            let (stem, ext) = split_cpm_name(name);
            for (i, ch) in stem.bytes().take(8).enumerate() {
                self.write8(address + 1 + i as u16, ch);
            }
            for (i, ch) in ext.bytes().take(3).enumerate() {
                self.write8(address + 9 + i as u16, ch);
            }
        }
    }

    fn fcb_host_path(&self, address: u16) -> Option<PathBuf> {
        let name = read_trimmed_cpm_field(&self.mem[address as usize + 1..address as usize + 9]);
        let ext = read_trimmed_cpm_field(&self.mem[address as usize + 9..address as usize + 12]);
        if name.is_empty() {
            return None;
        }
        let file = if ext.is_empty() {
            name
        } else {
            format!("{name}.{ext}")
        };
        Some(resolve_case_insensitive(&self.host_dir, &file))
    }

    fn return_byte(&mut self, cpu: &mut Cpu, value: u8) {
        cpu.registers().set_a(value);
        cpu.registers().set8(Reg8::L, value);
    }

    fn read8(&self, address: u16) -> u8 {
        self.mem[address as usize]
    }

    fn write8(&mut self, address: u16, value: u8) {
        self.mem[address as usize] = value;
    }
}

impl Machine for CpmMachine {
    fn peek(&self, address: u32) -> u8 {
        self.mem[address as u16 as usize]
    }

    fn poke(&mut self, address: u32, value: u8) {
        self.mem[address as u16 as usize] = value;
    }

    fn use_cycles(&self, _cycles: i32) {}

    fn port_in(&mut self, _address: u16) -> u8 {
        0
    }

    fn port_out(&mut self, _address: u16, _value: u8) {}
}

fn split_cpm_name(name: &str) -> (String, String) {
    let name = if name.as_bytes().get(1) == Some(&b':') {
        &name[2..]
    } else {
        name
    };
    let mut parts = name.splitn(2, '.');
    let stem = parts.next().unwrap_or_default().to_ascii_uppercase();
    let ext = parts.next().unwrap_or_default().to_ascii_uppercase();
    (stem, ext)
}

fn read_trimmed_cpm_field(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes)
        .trim_end_matches(' ')
        .to_string()
}

fn resolve_case_insensitive(dir: &Path, name: &str) -> PathBuf {
    let exact = dir.join(name);
    if exact.exists() {
        return exact;
    }
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let file_name = entry.file_name();
            if file_name.to_string_lossy().eq_ignore_ascii_case(name) {
                return entry.path();
            }
        }
    }
    exact
}

fn read_console_byte() -> Result<u8, String> {
    let mut byte = [0];
    io::stdin()
        .read_exact(&mut byte)
        .map_err(|e| e.to_string())?;
    Ok(if byte[0] == b'\n' { b'\r' } else { byte[0] })
}

fn write_console_byte(value: u8) -> Result<(), String> {
    if value == 0 {
        return Ok(());
    }
    let mut stdout = io::stdout();
    stdout.write_all(&[value]).map_err(|e| e.to_string())?;
    stdout.flush().map_err(|e| e.to_string())
}
