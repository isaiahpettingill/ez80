use super::decoder_8080::*;
use super::decoder_ez80::*;
use super::decoder_z80::*;
use super::environment::*;
use super::fast_bus::FastBus;
use super::fast_core::{step_ez80_fast, step_z80_fast};
use super::machine::*;
use super::opcode::*;
use super::registers::*;
use super::state::*;

const NMI_ADDRESS: u32 = 0x0066;

/// The Z80 cpu emulator.
///
/// Executes Z80 instructions changing the cpu State and Machine
pub struct Cpu {
    pub state: State,
    trace: bool,
    decoder: Box<dyn Decoder>,
    cycles: u64,
    fast_mode: FastMode,
    mode: CpuMode,
}

/// CPU family/mode selected for instruction decoding.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CpuMode {
    I8080,
    I8085,
    Z80,
    Z80N,
    Z180,
    EZ80,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum FastMode {
    Z80,
    EZ80,
    I8080,
}

pub(crate) trait Decoder {
    fn decode(&self, env: &mut Environment) -> &Opcode;
}

impl Cpu {
    /// Returns a Z80 Cpu instance. Alias of new_z80()
    pub fn new() -> Cpu {
        Self::new_z80()
    }

    /// Returns a Z80 Cpu instance
    pub fn new_z80() -> Cpu {
        Cpu {
            state: State::new(),
            trace: false,
            decoder: Box::new(DecoderZ80::new()),
            cycles: 0,
            fast_mode: FastMode::Z80,
            mode: CpuMode::Z80,
        }
    }

    /// Returns a ZX Spectrum Next Z80N-compatible CPU instance.
    ///
    /// The Z80N mode runs the documented Z80-compatible instruction set plus
    /// implemented ZX Spectrum Next extension opcodes.
    pub fn new_z80n() -> Cpu {
        Cpu {
            state: State::new(),
            trace: false,
            decoder: Box::new(DecoderZ80::new_z80n()),
            cycles: 0,
            fast_mode: FastMode::Z80,
            mode: CpuMode::Z80N,
        }
    }

    pub fn new_ez80() -> Cpu {
        Cpu {
            state: State::new(),
            trace: false,
            decoder: Box::new(DecoderEZ80::new()),
            cycles: 0,
            fast_mode: FastMode::EZ80,
            mode: CpuMode::EZ80,
        }
    }

    /// Returns a Zilog Z180 CPU instance.
    pub fn new_z180() -> Cpu {
        Cpu {
            state: State::new(),
            trace: false,
            decoder: Box::new(DecoderZ80::new_z180()),
            cycles: 0,
            fast_mode: FastMode::Z80,
            mode: CpuMode::Z180,
        }
    }

    /// Returns an Intel 8080 Cpu instance
    pub fn new_8080() -> Cpu {
        let mut cpu = Cpu {
            state: State::new(),
            trace: false,
            decoder: Box::new(Decoder8080::new()),
            cycles: 0,
            fast_mode: FastMode::I8080,
            mode: CpuMode::I8080,
        };

        cpu.state.reg.set_8080();
        cpu
    }

    /// Returns an Intel 8085 CPU instance.
    pub fn new_8085() -> Cpu {
        Cpu {
            mode: CpuMode::I8085,
            ..Self::new_8080()
        }
    }

    /// Returns a CPU configured for the requested mode.
    pub fn new_for_mode(mode: CpuMode) -> Cpu {
        match mode {
            CpuMode::I8080 => Self::new_8080(),
            CpuMode::I8085 => Self::new_8085(),
            CpuMode::Z80 => Self::new_z80(),
            CpuMode::Z80N => Self::new_z80n(),
            CpuMode::Z180 => Self::new_z180(),
            CpuMode::EZ80 => Self::new_ez80(),
        }
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

impl Cpu {
    /// Returns a clone of CPU-visible execution state.
    pub fn save_state(&self) -> State {
        self.state.clone()
    }

    /// Restores CPU-visible execution state.
    pub fn load_state(&mut self, state: &State) {
        self.state = state.clone();
    }

    /// Returns the fast-path CPU cycle counter.
    pub fn cycles(&self) -> u64 {
        self.cycles
    }

    /// Returns the selected CPU mode.
    pub fn mode(&self) -> CpuMode {
        self.mode
    }

    /// Sets the fast-path CPU cycle counter.
    pub fn set_cycles(&mut self, cycles: u64) {
        self.cycles = cycles;
    }

    /// Executes one instruction through the generic static-dispatch eZ80 fast path.
    pub fn step_fast<B: FastBus>(&mut self, bus: &mut B) {
        match self.fast_mode {
            FastMode::Z80 => step_z80_fast(&mut self.state, &mut self.cycles, bus),
            FastMode::EZ80 => step_ez80_fast(&mut self.state, &mut self.cycles, bus),
            FastMode::I8080 => {
                panic!("step_fast currently supports Cpu::new_z80() and Cpu::new_ez80()")
            }
        }
    }

    /// Runs until at least `cycles` additional fast-path CPU cycles have elapsed.
    pub fn run_cycles<B: FastBus>(&mut self, bus: &mut B, cycles: u64) {
        let target = self.cycles.wrapping_add(cycles);
        self.run_until(bus, target);
    }

    /// Runs until the fast-path CPU cycle counter reaches `target_cycle`.
    pub fn run_until<B: FastBus>(&mut self, bus: &mut B, target_cycle: u64) {
        while self.cycles < target_cycle && !self.is_halted() {
            let before = self.cycles;
            self.step_fast(bus);
            if self.cycles == before {
                break;
            }
        }
    }

    // Executes a single instruction
    //
    // Alternative to execute_instruction(), used by Fab Agon Emulator for
    // sake of a little efficiency. Ignores some ez80 emulator features not
    // used by FAE: reset_pending and nmi_pending
    pub fn fast_execute_instruction(&mut self, sys: &mut dyn Machine) {
        if self.is_halted() {
            // The CPU is in HALT state. Only interrupts can execute.
            return;
        }

        let mut env = Environment::new(&mut self.state, sys);
        let opcode = self.decoder.decode(&mut env);
        env.state.reg.begin_instruction_flags();
        opcode.execute(&mut env);
        env.state.reg.finish_instruction_flags();
        env.clear_index();
        env.state.clear_sz_prefix();
        env.state.instructions_executed += 1;
        increment_refresh_register(env.state);
    }

    /// Executes a single instruction
    ///
    /// # Arguments
    ///
    /// * `sys` - A representation of the emulated machine that has the Machine trait
    ///
    pub fn execute_instruction(&mut self, sys: &mut dyn Machine) {
        if self.is_halted() {
            // The CPU is in HALT state. Only interrupts can execute.
            return;
        }

        let mut env = Environment::new(&mut self.state, sys);
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
            env.subroutine_call(NMI_ADDRESS);
            if had_madl && was_z80 {
                env.push_byte_spl(0x01);
                env.state.reg.adl = true;
            }
        }

        let pc = env.state.pc();
        let opcode = self.decoder.decode(&mut env);
        if self.trace {
            print!("==> {:06x}: {:20}", pc, opcode.disasm(&env).0);
        }
        env.state.reg.begin_instruction_flags();
        opcode.execute(&mut env);
        env.state.reg.finish_instruction_flags();
        env.clear_index();
        env.state.clear_sz_prefix();
        env.state.instructions_executed += 1;
        increment_refresh_register(env.state);

        if self.trace {
            print!(" PC:{:06x} AF:{:04x} BC:{:06x} DE:{:06x} HL:{:06x} SPS:{:04x} SPL:{:06x} IX:{:06x} IY:{:06x} MB {:02x} ADL {:01x} MADL {:01x} tick {}",
                self.state.pc(),
                self.state.reg.get16(Reg16::AF),
                self.state.reg.get24(Reg16::BC),
                self.state.reg.get24(Reg16::DE),
                self.state.reg.get24(Reg16::HL),
                self.state.reg.get16(Reg16::SP),
                self.state.reg.get24(Reg16::SP),
                self.state.reg.get24(Reg16::IX),
                self.state.reg.get24(Reg16::IY),
                self.state.reg.mbase,
                self.state.reg.adl as i32,
                self.state.reg.madl as i32,
                self.state.instructions_executed,
            );
            println!(
                " [{:02x} {:02x} {:02x} {:02x}]",
                sys.peek(pc),
                sys.peek(pc.wrapping_add(1)),
                sys.peek(pc.wrapping_add(2)),
                sys.peek(pc.wrapping_add(3))
            );
        }
    }

    /// Returns the instrction in PC disassembled. PC is advanced.
    ///
    /// # Arguments
    ///
    /// * `sys` - A representation of the emulated machine that has the Machine trait
    ///  
    pub fn disasm_instruction(&mut self, sys: &mut dyn Machine) -> String {
        let mut env = Environment::new(&mut self.state, sys);
        let opcode = self.decoder.decode(&mut env);
        let (asm, pc_inc) = opcode.disasm(&env);
        for _ in 0..pc_inc {
            env.advance_pc();
        }
        asm
    }

    /// Activates or deactivates traces of the instruction executed and
    /// the state of the registers.
    ///
    /// # Arguments
    ///
    /// * `trace` - A bool defining the trace state to set
    pub fn set_trace(&mut self, trace: bool) {
        self.trace = trace;
    }

    /// Set eZ80 ADL state
    pub fn set_adl(&mut self, adl: bool) {
        self.state.reg.adl = adl;
    }

    /// Returns a Registers struct to read and write on the Z80 registers
    pub fn registers(&mut self) -> &mut Registers {
        &mut self.state.reg
    }

    /// Returns if the Cpu has executed a HALT
    pub fn is_halted(&self) -> bool {
        self.state.halted && !self.state.nmi_pending && !self.state.reset_pending
    }

    /// Non maskable interrupt request
    pub fn signal_nmi(&mut self) {
        self.state.nmi_pending = true
    }

    /// Signal reset
    pub fn signal_reset(&mut self) {
        self.state.reset_pending = true
    }
}

fn increment_refresh_register(state: &mut State) {
    let r = state.reg.get8(Reg8::R);
    state
        .reg
        .set8(Reg8::R, (r & 0x80) | r.wrapping_add(1) & 0x7f);
}
