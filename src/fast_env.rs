use super::fast_bus::FastBus;
use super::registers::*;
use super::state::{SizePrefix, State};

pub(crate) struct FastEnv<'a, B: FastBus> {
    pub state: &'a mut State,
    bus: &'a mut B,
    cycles: &'a mut u64,
    pending_cycles: i64,
}

impl<'a, B: FastBus> FastEnv<'a, B> {
    pub(crate) fn new(state: &'a mut State, bus: &'a mut B, cycles: &'a mut u64) -> Self {
        Self {
            state,
            bus,
            cycles,
            pending_cycles: 0,
        }
    }

    pub(crate) fn finish(self) {
        debug_assert!(self.pending_cycles >= 0);
        let cycles = self.pending_cycles.max(0) as u64;
        *self.cycles = self.cycles.wrapping_add(cycles);
        self.bus.add_cycles(cycles as u32);
    }

    pub(crate) fn use_cycles(&mut self, cycles: i32) {
        self.pending_cycles += cycles as i64;
    }

    pub(crate) fn wrap_address24(&self, address: u32, increment: i32) -> u32 {
        address.wrapping_add(increment as u32)
    }

    pub(crate) fn wrap_address16(&self, address: u32, increment: i32) -> u32 {
        (address & 0xff0000) + (address as u16).wrapping_add(increment as u16) as u32
    }

    pub(crate) fn wrap_address(&self, address: u32, increment: i32) -> u32 {
        if self.state.is_op_long() {
            self.wrap_address24(address, increment)
        } else {
            self.wrap_address16(address, increment)
        }
    }

    pub(crate) fn read8(&mut self, address: u32) -> u8 {
        self.use_cycles(1);
        self.bus.read8(address)
    }

    pub(crate) fn write8(&mut self, address: u32, value: u8) {
        self.use_cycles(1);
        self.bus.write8(address, value);
    }

    pub(crate) fn input8(&mut self, port: u16) -> u8 {
        self.use_cycles(1);
        self.bus.input8(port)
    }

    pub(crate) fn output8(&mut self, port: u16, value: u8) {
        self.use_cycles(1);
        self.bus.output8(port, value);
    }

    pub(crate) fn read16(&mut self, address: u32) -> u16 {
        self.read8(address) as u16 + ((self.read8(self.wrap_address(address, 1)) as u16) << 8)
    }

    pub(crate) fn write16(&mut self, address: u32, value: u16) {
        self.write8(address, value as u8);
        self.write8(self.wrap_address(address, 1), (value >> 8) as u8);
    }

    pub(crate) fn read24(&mut self, address: u32) -> u32 {
        self.read8(address) as u32
            + ((self.read8(self.wrap_address(address, 1)) as u32) << 8)
            + ((self.read8(self.wrap_address(address, 2)) as u32) << 16)
    }

    pub(crate) fn write24(&mut self, address: u32, value: u32) {
        self.write8(address, value as u8);
        self.write8(self.wrap_address(address, 1), (value >> 8) as u8);
        self.write8(self.wrap_address(address, 2), (value >> 16) as u8);
    }

    pub(crate) fn advance_pc(&mut self) -> u8 {
        let pc = self.state.pc();
        let value = self.read8(pc);
        if self.state.reg.adl {
            self.state.set_pc(self.wrap_address24(pc, 1));
        } else {
            self.state.set_pc(self.wrap_address16(pc, 1));
        }
        value
    }

    pub(crate) fn advance_immediate16(&mut self) -> u16 {
        let mut value = self.advance_pc() as u16;
        value += (self.advance_pc() as u16) << 8;
        value
    }

    pub(crate) fn advance_immediate24(&mut self) -> u32 {
        let mut value = self.advance_pc() as u32;
        value += (self.advance_pc() as u32) << 8;
        value += (self.advance_pc() as u32) << 16;
        value
    }

    pub(crate) fn advance_immediate16or24(&mut self) -> u32 {
        if self.state.is_imm_long() {
            self.advance_immediate24()
        } else {
            self.advance_immediate16() as u32
        }
    }

    pub(crate) fn advance_immediate_16mbase_or_24(&mut self) -> u32 {
        let imm = self.advance_immediate16or24();
        if self.state.is_op_long() {
            imm
        } else {
            (imm & 0xffff) + ((self.state.reg.mbase as u32) << 16)
        }
    }

    pub(crate) fn set_index(&mut self, index: Reg16) {
        self.state.index = index;
    }

    pub(crate) fn clear_index(&mut self) {
        self.state.index = Reg16::HL;
        self.state.displacement = 0;
    }

    pub(crate) fn is_alt_index(&self) -> bool {
        self.state.index == Reg16::IX || self.state.index == Reg16::IY
    }

    pub(crate) fn get_index(&self) -> Reg16 {
        self.state.index
    }

    pub(crate) fn load_displacement(&mut self) {
        self.state.displacement = self.advance_pc() as i8;
    }

    pub(crate) fn trap_illegal_instruction(&mut self) {
        self.state.illegal_instruction = true;
        self.state.illegal_instruction_adl = self.state.reg.adl;
        self.state.halted = true;
    }

    pub(crate) fn index_value(&self) -> u32 {
        if self.state.is_op_long() {
            self.state.reg.get24(self.state.index)
        } else {
            self.state.reg.get16_mbase(self.state.index)
        }
    }

    pub(crate) fn index_address(&self) -> u32 {
        let address = if self.state.is_op_long() {
            self.state.reg.get24(self.state.index)
        } else {
            self.state.reg.get16_mbase(self.state.index)
        };
        if self.is_alt_index() {
            (address as i32).wrapping_add(self.state.displacement as i32) as u32
        } else {
            address
        }
    }

    fn translate_reg(&self, reg: Reg8) -> Reg8 {
        match self.state.index {
            Reg16::IX => match reg {
                Reg8::H => Reg8::IXH,
                Reg8::L => Reg8::IXL,
                _ => reg,
            },
            Reg16::IY => match reg {
                Reg8::H => Reg8::IYH,
                Reg8::L => Reg8::IYL,
                _ => reg,
            },
            _ => reg,
        }
    }

    pub(crate) fn reg8_ext(&mut self, reg: Reg8) -> u8 {
        if reg == Reg8::_HL {
            self.read8(self.index_address())
        } else {
            self.state.reg.get8(self.translate_reg(reg))
        }
    }

    pub(crate) fn set_reg(&mut self, reg: Reg8, value: u8) {
        if reg == Reg8::_HL {
            self.write8(self.index_address(), value);
        } else {
            self.state.reg.set8(self.translate_reg(reg), value);
        }
    }

    pub(crate) fn reg16mbase_or_24(&self, rr: Reg16) -> u32 {
        if self.state.is_op_long() {
            self.state.reg.get24(rr)
        } else {
            self.state.reg.get16_mbase(rr)
        }
    }

    pub(crate) fn reg16or24_ext(&self, rr: Reg16) -> u32 {
        if self.state.is_op_long() {
            if rr == Reg16::HL {
                self.state.reg.get24(self.state.index)
            } else if rr == Reg16::AF {
                self.state.reg.get16(rr) as u32
            } else {
                self.state.reg.get24(rr)
            }
        } else if rr == Reg16::HL {
            self.state.reg.get16(self.state.index) as u32
        } else {
            self.state.reg.get16(rr) as u32
        }
    }

    pub(crate) fn set_reg16or24(&mut self, rr: Reg16, value: u32) {
        if self.state.is_op_long() {
            self.set_reg24(rr, value);
        } else {
            self.set_reg16(rr, value as u16);
        }
    }

    pub(crate) fn set_reg16(&mut self, rr: Reg16, value: u16) {
        if rr == Reg16::HL {
            self.state.reg.set16(self.state.index, value);
        } else {
            self.state.reg.set16(rr, value);
        }
    }

    pub(crate) fn set_reg16_preserve_17_to_24(&mut self, rr: Reg16, value: u16) {
        if rr == Reg16::HL {
            self.state
                .reg
                .set16_preserve_17_to_24(self.state.index, value);
        } else {
            self.state.reg.set16_preserve_17_to_24(rr, value);
        }
    }

    pub(crate) fn set_reg24(&mut self, rr: Reg16, value: u32) {
        if rr == Reg16::HL {
            self.state.reg.set24(self.state.index, value);
        } else {
            self.state.reg.set24(rr, value);
        }
    }

    pub(crate) fn push_byte_sps(&mut self, value: u8) {
        let sps = self.wrap_address16(self.state.reg.get16_mbase(Reg16::SP), -1);
        self.write8(sps, value);
        self.state.reg.set16(Reg16::SP, sps as u16);
    }

    pub(crate) fn pop_byte_sps(&mut self) -> u8 {
        let sps = self.state.reg.get16_mbase(Reg16::SP);
        let value = self.read8(sps);
        self.state
            .reg
            .set16(Reg16::SP, self.wrap_address16(sps, 1) as u16);
        value
    }

    pub(crate) fn push_byte_spl(&mut self, value: u8) {
        let spl = self.wrap_address24(self.state.reg.get24(Reg16::SP), -1);
        self.write8(spl, value);
        self.state.reg.set24(Reg16::SP, spl);
    }

    pub(crate) fn pop_byte_spl(&mut self) -> u8 {
        let spl = self.state.reg.get24(Reg16::SP);
        let value = self.read8(spl);
        self.state.reg.set24(Reg16::SP, self.wrap_address24(spl, 1));
        value
    }

    pub(crate) fn push(&mut self, value: u32) {
        let u = (value >> 16) as u8;
        let h = (value >> 8) as u8;
        let l = value as u8;
        if self.state.is_op_long() {
            self.push_byte_spl(u);
            self.push_byte_spl(h);
            self.push_byte_spl(l);
        } else {
            self.push_byte_sps(h);
            self.push_byte_sps(l);
        }
    }

    pub(crate) fn pop(&mut self) -> u32 {
        let l;
        let h;
        let u;
        if self.state.is_op_long() {
            l = self.pop_byte_spl();
            h = self.pop_byte_spl();
            u = self.pop_byte_spl();
        } else {
            l = self.pop_byte_sps();
            h = self.pop_byte_sps();
            u = 0;
        }
        (l as u32) + ((h as u32) << 8) + ((u as u32) << 16)
    }

    pub(crate) fn subroutine_return(&mut self) {
        if self.state.reg.adl {
            match self.state.sz_prefix {
                SizePrefix::None => {
                    let pc = self.pop();
                    self.state.set_pc(pc);
                }
                SizePrefix::LIL | SizePrefix::LIS => {
                    let adl_flag = self.pop_byte_spl();
                    if adl_flag & 1 == 1 {
                        let address = self.pop();
                        self.state.set_pc(address);
                    } else {
                        let mut address = self.pop_byte_spl() as u32;
                        address += (self.pop_byte_spl() as u32) << 8;
                        self.state.set_pc(address);
                        self.state.reg.adl = false;
                    }
                }
                _ => {
                    let pc = self.pop();
                    self.state.set_pc(pc);
                }
            }
        } else {
            match self.state.sz_prefix {
                SizePrefix::None => {
                    let pc = self.pop();
                    self.state.set_pc(pc);
                }
                SizePrefix::LIL | SizePrefix::SIL => {
                    let adl_flag = self.pop_byte_spl();
                    if adl_flag & 1 == 1 {
                        let address = self.pop();
                        self.state.set_pc(address);
                        self.state.reg.adl = true;
                    } else {
                        let mut address = self.pop_byte_spl() as u32;
                        address += (self.pop_byte_spl() as u32) << 8;
                        self.state.set_pc(address);
                    }
                }
                _ => {
                    let pc = self.pop();
                    self.state.set_pc(pc);
                }
            }
        }
    }
}
