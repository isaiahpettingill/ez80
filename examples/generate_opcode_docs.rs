use ez80::{Cpu, CpuMode, Machine};
use std::fmt::Write as _;
use std::panic::{catch_unwind, AssertUnwindSafe};

struct ModeDoc {
    mode: CpuMode,
    file: &'static str,
    title: &'static str,
    adl: bool,
    include_z80_prefixes: bool,
    include_indexed: bool,
    include_ez80_sizes: bool,
}

struct Form {
    title: String,
    prefixes: Vec<u8>,
    opcodes: Vec<Vec<u8>>,
}

struct DocMachine {
    mem: Vec<u8>,
}

impl DocMachine {
    fn new(bytes: &[u8]) -> Self {
        let mut mem = vec![0; 16];
        for (i, byte) in bytes.iter().copied().enumerate() {
            mem[i] = byte;
        }
        Self { mem }
    }
}

impl Machine for DocMachine {
    fn peek(&self, address: u32) -> u8 {
        self.mem.get(address as usize).copied().unwrap_or(0)
    }

    fn poke(&mut self, address: u32, value: u8) {
        if let Some(byte) = self.mem.get_mut(address as usize) {
            *byte = value;
        }
    }

    fn use_cycles(&self, _cycles: i32) {}

    fn port_in(&mut self, _address: u16) -> u8 {
        0
    }

    fn port_out(&mut self, _address: u16, _value: u8) {}
}

fn main() -> std::io::Result<()> {
    let modes = [
        ModeDoc {
            mode: CpuMode::I8080,
            file: "docs/opcodes_8080.md",
            title: "Intel 8080 Opcodes",
            adl: false,
            include_z80_prefixes: false,
            include_indexed: false,
            include_ez80_sizes: false,
        },
        ModeDoc {
            mode: CpuMode::I8085,
            file: "docs/opcodes_8085.md",
            title: "Intel 8085 Opcodes",
            adl: false,
            include_z80_prefixes: false,
            include_indexed: false,
            include_ez80_sizes: false,
        },
        ModeDoc {
            mode: CpuMode::Z80,
            file: "docs/opcodes_z80.md",
            title: "Zilog Z80 Opcodes",
            adl: false,
            include_z80_prefixes: true,
            include_indexed: true,
            include_ez80_sizes: false,
        },
        ModeDoc {
            mode: CpuMode::Z80N,
            file: "docs/opcodes_z80n.md",
            title: "ZX Spectrum Next Z80N Opcodes",
            adl: false,
            include_z80_prefixes: true,
            include_indexed: true,
            include_ez80_sizes: false,
        },
        ModeDoc {
            mode: CpuMode::Z180,
            file: "docs/opcodes_z180.md",
            title: "Zilog Z180 Opcodes",
            adl: false,
            include_z80_prefixes: true,
            include_indexed: true,
            include_ez80_sizes: false,
        },
        ModeDoc {
            mode: CpuMode::EZ80,
            file: "docs/opcodes_ez80.md",
            title: "Zilog eZ80 Opcodes",
            adl: true,
            include_z80_prefixes: true,
            include_indexed: true,
            include_ez80_sizes: true,
        },
    ];

    std::panic::set_hook(Box::new(|_| {}));
    std::fs::create_dir_all("docs")?;
    for mode in modes {
        std::fs::write(mode.file, render_mode(&mode))?;
    }

    Ok(())
}

fn render_mode(mode: &ModeDoc) -> String {
    let mut out = String::new();
    writeln!(out, "# {}", mode.title).unwrap();
    writeln!(out).unwrap();
    writeln!(
        out,
        "Generated from the emulator disassembler for `CpuMode::{:?}`.",
        mode.mode
    )
    .unwrap();
    writeln!(
        out,
        "The tables list every opcode form decoded by this mode. `NONINOP` is the emulator's non-instruction NOP behavior for unsupported ED-prefixed Z80-family opcodes."
    )
    .unwrap();
    writeln!(out).unwrap();

    for form in forms(mode) {
        writeln!(out, "## {}", form.title).unwrap();
        writeln!(out).unwrap();
        writeln!(out, "| Bytes | Disassembly |").unwrap();
        writeln!(out, "| --- | --- |").unwrap();
        for opcode in form.opcodes {
            let bytes = build_bytes(&form.prefixes, &opcode);
            let (asm, used) = disasm(mode, &bytes);
            writeln!(out, "| `{}` | `{}` |", hex_bytes(&bytes[..used]), asm).unwrap();
        }
        writeln!(out).unwrap();
    }

    out
}

fn forms(mode: &ModeDoc) -> Vec<Form> {
    let mut forms = vec![Form {
        title: "Base opcodes".to_string(),
        prefixes: vec![],
        opcodes: byte_forms(),
    }];

    if mode.include_z80_prefixes {
        forms.extend([
            Form {
                title: "CB-prefixed opcodes".to_string(),
                prefixes: vec![0xcb],
                opcodes: byte_forms(),
            },
            Form {
                title: "ED-prefixed opcodes".to_string(),
                prefixes: vec![0xed],
                opcodes: byte_forms(),
            },
        ]);
    }

    if mode.include_indexed {
        forms.extend([
            Form {
                title: "DD-prefixed IX opcodes".to_string(),
                prefixes: vec![0xdd],
                opcodes: byte_forms(),
            },
            Form {
                title: "FD-prefixed IY opcodes".to_string(),
                prefixes: vec![0xfd],
                opcodes: byte_forms(),
            },
            Form {
                title: "DD CB indexed bit opcodes".to_string(),
                prefixes: vec![0xdd, 0xcb],
                opcodes: indexed_cb_forms(),
            },
            Form {
                title: "FD CB indexed bit opcodes".to_string(),
                prefixes: vec![0xfd, 0xcb],
                opcodes: indexed_cb_forms(),
            },
        ]);
    }

    if mode.include_ez80_sizes {
        for (prefix, name) in [(0x40, "SIS"), (0x49, "LIS"), (0x52, "SIL"), (0x5b, "LIL")] {
            forms.extend([
                Form {
                    title: format!("eZ80 {name} base opcodes"),
                    prefixes: vec![prefix],
                    opcodes: byte_forms(),
                },
                Form {
                    title: format!("eZ80 {name} ED-prefixed opcodes"),
                    prefixes: vec![prefix, 0xed],
                    opcodes: byte_forms(),
                },
            ]);
        }
    }

    forms
}

fn byte_forms() -> Vec<Vec<u8>> {
    (0u16..=255).map(|value| vec![value as u8]).collect()
}

fn indexed_cb_forms() -> Vec<Vec<u8>> {
    (0u16..=255).map(|value| vec![0x01, value as u8]).collect()
}

fn build_bytes(prefixes: &[u8], opcode: &[u8]) -> Vec<u8> {
    let mut bytes = prefixes.to_vec();
    bytes.extend_from_slice(opcode);
    bytes.extend_from_slice(&[0x34, 0x12, 0x56]);
    if prefixes.ends_with(&[0xcb]) && opcode.len() == 2 {
        bytes = prefixes[..prefixes.len() - 1].to_vec();
        bytes.extend_from_slice(&[0xcb, opcode[0], opcode[1], 0x34, 0x12, 0x56]);
    }
    bytes
}

fn disasm(mode: &ModeDoc, bytes: &[u8]) -> (String, usize) {
    let result = catch_unwind(AssertUnwindSafe(|| {
        let mut cpu = Cpu::new_for_mode(mode.mode);
        cpu.set_adl(mode.adl);
        let mut machine = DocMachine::new(bytes);
        cpu.state.set_pc(0);
        let asm = cpu.disasm_instruction(&mut machine);
        (asm, cpu.state.pc() as usize)
    }));

    match result {
        Ok(entry) => entry,
        Err(_) => ("panic".to_string(), bytes.len()),
    }
}

fn hex_bytes(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<Vec<_>>()
        .join(" ")
}
