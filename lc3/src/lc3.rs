//
// lc3-vm, a virtual machine for the LC-3 (Little Computer 3) architecture.
// Copyright (C) 2024  Fares A. Bakhit
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//

use crate::ImageFile;
use crate::InstructionDecode;
use crate::IoDevice;
use crate::IoDeviceRegister;
use crate::Memory;
use crate::OpCode;
use crate::TrapCode;
use crate::{CondCodes, Reg, Registers};

use core::{fmt, slice};

/// LC-3 virtual machine.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Lc3<IO: IoDevice> {
    pub registers: Registers,
    pub memory: Memory<IO>,
}

impl<IO: IoDevice> Lc3<IO> {
    pub const TRAP_VECTOR_TABLE_START: u16 = 0x0000;
    pub const INTERRUPT_VECTOR_TABLE_START: u16 = 0x0100;
    pub const OPERATING_SYSTEM_START: u16 = 0x0200;
    pub const USER_PROGRAMS_START: u16 = 0x3000;

    /// Initialize a new LC-3 virtual machine with an [`IoDevice`][`crate::IoDevice`].
    pub const fn new(iodevice: IO) -> Lc3<IO> {
        Lc3 {
            registers: Registers::new(),
            memory: Memory::new(iodevice),
        }
    }

    /// Load an image from an [`ImageFile`][`crate::ImageFile`].
    pub fn load_image<F: ImageFile>(&mut self, file: &mut F) -> Result<(), F::Error> {
        file.load_image_into(self.memory.as_mut())
    }

    /// Run indefinitely at [`Self::USER_PROGRAMS_START`] until [`Self::should_halt`] returns true.
    pub fn run(&mut self) -> Result<(), Error<IO::Error>> {
        self.run_at(Self::USER_PROGRAMS_START)
    }

    // Run indefinitely at `addr` until [`Self::should_halt`] returns true.
    pub fn run_at(&mut self, addr: u16) -> Result<(), Error<IO::Error>> {
        self.run_common::<false>(addr)
    }

    /// Run indefinitely at [`Self::USER_PROGRAMS_START`] with trap emulated until [`Self::should_halt`] returns true.
    pub fn run_with_virtual_trap_vector_table(&mut self) -> Result<(), Error<IO::Error>> {
        // self.boot_with_virtual_trap_vector_table()?;
        self.run_with_virtual_trap_vector_table_at(Self::USER_PROGRAMS_START)
    }

    /// Run indefinitely with trap emulated at `addr` until [`Self::should_halt`] returns true.
    pub fn run_with_virtual_trap_vector_table_at(
        &mut self,
        addr: u16,
    ) -> Result<(), Error<IO::Error>> {
        self.run_common::<true>(addr)
    }

    fn run_common<const VIRT_TVT: bool>(&mut self, addr: u16) -> Result<(), Error<IO::Error>> {
        self.reset();
        self.registers.pc = addr;
        while !self.should_halt() {
            self.next_instruction_common::<VIRT_TVT>()?;
        }

        Ok(())
    }

    /// Execute next instruction.
    pub fn next_instruction(&mut self) -> Result<(), Error<IO::Error>> {
        self.next_instruction_common::<false>()
    }

    /// Execute next instruction with trap emulated.
    pub fn next_instruction_with_virtual_trap_vector_table(
        &mut self,
    ) -> Result<(), Error<IO::Error>> {
        self.next_instruction_common::<true>()
    }

    fn next_instruction_common<const VIRT_TVT: bool>(&mut self) -> Result<(), Error<IO::Error>> {
        let inst = self.memory.read(self.registers.pc);

        // All instructions with a PC offset parameter
        // require PC to be incremented.
        self.registers.pc = self.registers.pc.wrapping_add(1);

        match inst.opcode() {
            OpCode::Add => self.add(inst),
            OpCode::And => self.and(inst),
            OpCode::Not => self.not(inst),
            OpCode::Br => self.br(inst),
            OpCode::Jmp => self.jmp(inst),
            OpCode::Jsr => self.jsr(inst),
            OpCode::Ld => self.ld(inst),
            OpCode::Ldi => self.ldi(inst),
            OpCode::Ldr => self.ldr(inst),
            OpCode::Lea => self.lea(inst),
            OpCode::St => self.st(inst),
            OpCode::Sti => self.sti(inst),
            OpCode::Str => self.str(inst),
            OpCode::Trap if VIRT_TVT => self.trap_emulated(inst)?,
            OpCode::Trap => self.trap(inst),
            OpCode::Rti | OpCode::Res => return Err(Error::OpCodeNotImplemented),
        }

        Ok(())
    }

    /// Returns true iff the clock enable bit of [`IoDeviceRegister::Mcr`] is cleared.
    pub fn should_halt(&mut self) -> bool {
        !self.memory.read(IoDeviceRegister::Mcr as u16).isbitset(15)
    }

    /// Clears the clock enable bit of [`IoDeviceRegister::Mcr`].
    pub fn halt(&mut self) {
        self.memory.write(
            IoDeviceRegister::Mcr as u16,
            IoDeviceRegister::STATUS_DECLINE,
        );
    }

    /// Turns on the clock enable bit of [`IoDeviceRegister::Mcr`].
    pub fn reset(&mut self) {
        self.memory.write(
            IoDeviceRegister::Mcr as u16,
            IoDeviceRegister::STATUS_ACCEPT,
        );
    }
}

impl<IO: IoDevice> Lc3<IO> {
    fn add(&mut self, inst: u16) {
        let dr = inst.reg1();
        let sr1 = inst.reg2();
        if inst.isbitset(5) {
            let imm5 = inst.imm5();
            let value = self.registers[sr1].wrapping_add(imm5);
            self.registers[dr] = value;
        } else {
            let sr2 = inst.reg3();
            let value = self.registers[sr1].wrapping_add(self.registers[sr2]);
            self.registers[dr] = value;
        }
        self.setcc(dr);
    }

    fn and(&mut self, inst: u16) {
        let dr = inst.reg1();
        let sr1 = inst.reg2();
        if inst.isbitset(5) {
            let imm5 = inst.imm5();
            self.registers[dr] = self.registers[sr1] & imm5;
        } else {
            let sr2 = inst.reg3();
            self.registers[dr] = self.registers[sr1] & self.registers[sr2];
        }
        self.setcc(dr);
    }

    fn not(&mut self, inst: u16) {
        let dr = inst.reg1();
        let sr = inst.reg2();
        self.registers[dr] = !self.registers[sr];
        self.setcc(dr);
    }

    fn br(&mut self, inst: u16) {
        let cc = inst.condcodes();
        if self.registers.cc.intersects(cc) {
            let value = self.registers.pc.wrapping_add(inst.imm9());
            self.registers.pc = value;
        }
    }

    fn jmp(&mut self, inst: u16) {
        let baser = inst.reg2();
        self.registers.pc = self.registers[baser];
    }

    fn jsr(&mut self, inst: u16) {
        self.registers.r7 = self.registers.pc;
        if inst.isbitset(11) {
            let value = self.registers.pc.wrapping_add(inst.imm11());
            self.registers.pc = value;
        } else {
            let baser = inst.reg2();
            self.registers.pc = self.registers[baser];
        }
    }

    fn ld(&mut self, inst: u16) {
        let dr = inst.reg1();
        let addr = self.registers.pc.wrapping_add(inst.imm9());
        self.registers[dr] = self.memory.read(addr);
        self.setcc(dr);
    }

    fn ldi(&mut self, inst: u16) {
        let dr = inst.reg1();
        let addr = self.registers.pc.wrapping_add(inst.imm9());
        let addr = self.memory.read(addr);
        self.registers[dr] = self.memory.read(addr);
        self.setcc(dr);
    }

    fn ldr(&mut self, inst: u16) {
        let dr = inst.reg1();
        let baser = inst.reg2();
        let addr = self.registers[baser].wrapping_add(inst.imm6());
        self.registers[dr] = self.memory.read(addr);
        self.setcc(dr);
    }

    fn lea(&mut self, inst: u16) {
        let dr = inst.reg1();
        let value = self.registers.pc.wrapping_add(inst.imm9());
        self.registers[dr] = value;
        self.setcc(dr);
    }

    fn st(&mut self, inst: u16) {
        let sr = inst.reg1();
        let addr = self.registers.pc.wrapping_add(inst.imm9());
        self.memory.write(addr, self.registers[sr]);
    }

    fn sti(&mut self, inst: u16) {
        let sr = inst.reg1();
        let addr = self.registers.pc.wrapping_add(inst.imm9());
        let addr = self.memory.read(addr);
        self.memory.write(addr, self.registers[sr]);
    }

    fn str(&mut self, inst: u16) {
        let sr = inst.reg1();
        let baser = inst.reg2();
        let addr = self.registers[baser].wrapping_add(inst.imm6());
        self.memory.write(addr, self.registers[sr]);
    }

    fn setcc(&mut self, dr: Reg) {
        let result = self.registers[dr];
        self.registers.cc = CondCodes::from_signum(result);
    }

    fn trap(&mut self, inst: u16) {
        self.registers.r7 = self.registers.pc;
        self.registers.pc = self.memory.read(inst.imm8());
    }

    fn trap_emulated(&mut self, inst: u16) -> Result<(), Error<IO::Error>> {
        self.registers.r7 = self.registers.pc;

        let trapcode = match inst.trapcode() {
            Some(trapcode) => trapcode,
            None => {
                self.memory.io.write(b"UNDEFINED TRAP EXECUTED")?;
                return Ok(());
            }
        };

        match trapcode {
            TrapCode::Getc => {
                let mut byte = 0;
                self.memory.io.read(slice::from_mut(&mut byte))?;
                self.registers.r0 = byte as u16;
                self.setcc(Reg::R0);
            }
            TrapCode::Out => {
                let byte = self.registers.r0 as u8;
                self.memory.io.write(slice::from_ref(&byte))?;
                self.memory.io.flush()?;
            }
            TrapCode::Puts => {
                let mut sp = self.registers.r0;
                let mut byte = self.memory.read(sp) as u8;
                while byte != 0 {
                    self.memory.io.write(slice::from_ref(&byte))?;
                    sp += 1;
                    byte = self.memory.read(sp) as u8;
                }
                self.memory.io.flush()?;
            }
            TrapCode::In => {
                self.memory.io.write(b"Enter a character: ")?;
                let mut byte = 0;
                self.memory.io.read(slice::from_mut(&mut byte))?;
                self.memory.io.write(slice::from_ref(&byte))?;
                self.memory.io.flush()?;
                self.registers.r0 = byte as u16;
            }
            TrapCode::PutSp => unsafe {
                let sp = self.registers.r0;
                let start = self.memory.as_ref().as_ptr().add(sp as usize) as *const u8;
                let end = {
                    let mut end = start;
                    while *end != b'\0' {
                        end = end.add(1);
                    }
                    end
                };
                let len = end.offset_from(start) as usize;
                let slice = slice::from_raw_parts(start, len);
                self.memory.io.write(slice)?;
                self.memory.io.flush()?;
            },
            TrapCode::Halt => {
                self.memory.io.write(b"HALT\n")?;
                self.halt();
            }
        }

        Ok(())
    }
}

/// Error type for [`Lc3`] functions.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Error<IO> {
    Io(IO),
    OpCodeNotImplemented,
}

impl<IO: fmt::Display> fmt::Display for Error<IO> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(err) => err.fmt(f),
            Error::OpCodeNotImplemented => f.write_str("opcode not implemented."),
        }
    }
}

impl<IO> From<IO> for Error<IO> {
    fn from(value: IO) -> Self {
        Error::Io(value)
    }
}
