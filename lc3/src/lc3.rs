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
use crate::Memory;
use crate::OpCode;
use crate::TrapCode;
use crate::{CondCodes, Registers, GPR};
use core::slice;

/// LC-3 virtual machine.
pub struct LC3<IO: IoDevice> {
    registers: Registers,
    memory: Memory<IO>,
}

impl<IO: IoDevice> LC3<IO> {
    /// Initialize a new LC-3 virtual machine with an [`IoDevice`][`crate::IoDevice`].
    pub const fn new(iodevice: IO) -> LC3<IO> {
        LC3 {
            registers: Registers::new(),
            memory: Memory::new(iodevice),
        }
    }

    /// Load an image from an [`InputDevice`][`crate::InputDevice`].
    pub fn load_image<F: ImageFile>(&mut self, file: &mut F) -> Result<(), F::Error> {
        let mut origin: [u8; 2] = [0; 2];
        file.read(&mut origin)?;
        let origin = u16::from_be_bytes(origin) as usize;

        // Memory at `origin` as a slice of `u8` bytes
        let slice = {
            let data = &mut self.memory[origin] as *mut u16 as *mut u8;
            let len = (Memory::<IO>::LEN - origin) * 2;
            unsafe { slice::from_raw_parts_mut(data, len) }
        };

        let end = origin + file.read(slice)? / 2;

        //
        // Proof that end <= memory.len
        //
        // ImageFile::read guarantees 0 <= n <= slice.len().
        // Define end as origin+n/2.
        // Define slice.len as 2(memory.len-origin).
        //
        //
        // 0      <=           n    <=               slice.len
        // 0      <=           n/2  <=               slice.len        / 2
        // origin <= (origin + n/2) <=               slice.len        / 2
        // origin <= (origin + n/2) <= origin +      slice.len        / 2
        // origin <= (origin + n/2) <= origin + (memory.len - origin)
        // origin <=       end      <= memory.len
        //
        // Q.E.D.
        //

        self.memory[origin..end]
            .iter_mut()
            .for_each(|x| *x = u16::from_be(*x));

        Ok(())
    }

    /// Run indefinitely.
    pub fn run(&mut self) -> Result<(), Error<IO::Error>> {
        loop {
            if let Status::Halt = self.next_instruction()? {
                break Ok(());
            }
        }
    }

    /// Execute next instruction.
    pub fn next_instruction(&mut self) -> Result<Status, Error<IO::Error>> {
        let inst = self.memory[self.registers.pc];

        // All instructions with a PC offset parameter
        // require PC to be incremented.
        self.registers.pc = self.registers.pc.wrapping_add(1);

        match inst.opcode() {
            OpCode::ADD => self.add(inst),
            OpCode::AND => self.and(inst),
            OpCode::NOT => self.not(inst),
            OpCode::BR => self.br(inst),
            OpCode::JMP => self.jmp(inst),
            OpCode::JSR => self.jsr(inst),
            OpCode::LD => self.ld(inst),
            OpCode::LDI => self.ldi(inst),
            OpCode::LDR => self.ldr(inst),
            OpCode::LEA => self.lea(inst),
            OpCode::ST => self.st(inst),
            OpCode::STI => self.sti(inst),
            OpCode::STR => self.str(inst),
            OpCode::TRAP => return self.trap(inst),
            OpCode::RTI | OpCode::RES => return Err(Error::OpCodeNotImplemented),
        }

        Ok(Status::Continue)
    }
}

impl<IO: IoDevice> LC3<IO> {
    fn add(&mut self, inst: u16) {
        let dr = inst.reg1();
        let sr1 = inst.reg2();
        if inst.isbitset(5) {
            let imm5 = inst.imm5();
            let value = self.registers[sr1] as u32 + imm5 as u32;
            let value = value as u16;
            self.registers[dr] = value;
        } else {
            let sr2 = inst.reg3();
            let value = self.registers[sr1] as u32 + self.registers[sr2] as u32;
            let value = value as u16;
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
        if self.registers.cc.intersects(&cc) {
            let value = self.registers.pc as u32 + inst.imm9() as u32;
            let value = value as u16;
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
            let value = self.registers.pc as u32 + inst.imm11() as u32;
            let value = value as u16;
            self.registers.pc = value;
        } else {
            let baser = inst.reg2();
            self.registers.pc = self.registers[baser];
        }
    }

    fn ld(&mut self, inst: u16) {
        let dr = inst.reg1();
        let addr = self.registers.pc as u32 + inst.imm9() as u32;
        let addr = addr as u16;
        self.registers[dr] = self.memory[addr];
        self.setcc(dr);
    }

    fn ldi(&mut self, inst: u16) {
        let dr = inst.reg1();
        let addr = self.registers.pc as u32 + inst.imm9() as u32;
        let addr = addr as u16;
        let addr = self.memory[addr];
        self.registers[dr] = self.memory[addr];
        self.setcc(dr);
    }

    fn ldr(&mut self, inst: u16) {
        let dr = inst.reg1();
        let baser = inst.reg2();
        let addr = self.registers[baser] as u32 + inst.imm6() as u32;
        let addr = addr as u16;
        self.registers[dr] = self.memory[addr];
        self.setcc(dr);
    }

    fn lea(&mut self, inst: u16) {
        let dr = inst.reg1();
        let value = self.registers.pc as u32 + inst.imm9() as u32;
        let value = value as u16;
        self.registers[dr] = value;
        self.setcc(dr);
    }

    fn st(&mut self, inst: u16) {
        let sr = inst.reg1();
        let addr = self.registers.pc as u32 + inst.imm9() as u32;
        let addr = addr as u16;
        self.memory[addr] = self.registers[sr];
    }

    fn sti(&mut self, inst: u16) {
        let sr = inst.reg1();
        let addr = self.registers.pc as u32 + inst.imm9() as u32;
        let addr = addr as u16;
        let addr = self.memory[addr];
        self.memory[addr] = self.registers[sr];
    }

    fn str(&mut self, inst: u16) {
        let sr = inst.reg1();
        let baser = inst.reg2();
        let addr = self.registers[baser] as u32 + inst.imm6() as u32;
        let addr = addr as u16;
        self.memory[addr] = self.registers[sr];
    }

    fn setcc(&mut self, dr: GPR) {
        let result = self.registers[dr];
        self.registers.cc = CondCodes::from_signum(result);
    }

    fn trap(&mut self, inst: u16) -> Result<Status, Error<IO::Error>> {
        self.registers.r7 = self.registers.pc;

        let trapcode = match inst.trapcode() {
            Some(trapcode) => trapcode,
            None => {
                let msg = b"SYSTEM CALL NOT IMPLEMENTED ... HALTING.\n";
                self.memory.io.write(msg)?;
                return Err(Error::SystemCallNotImplemented);
            }
        };

        match trapcode {
            TrapCode::GETC => {
                let mut byte = 0;
                self.memory.io.read(slice::from_mut(&mut byte))?;
                self.registers.r0 = byte as u16;
                self.setcc(GPR::R0);
            }
            TrapCode::OUT => {
                let byte = self.registers.r0 as u8;
                self.memory.io.write(slice::from_ref(&byte))?;
                self.memory.io.flush()?;
            }
            TrapCode::PUTS => {
                let mut sp = self.registers.r0;
                let mut byte = self.memory[sp] as u8;
                while byte != 0 {
                    self.memory.io.write(slice::from_ref(&byte))?;
                    sp += 1;
                    byte = self.memory[sp] as u8;
                }
                self.memory.io.flush()?;
            }
            TrapCode::IN => {
                self.memory.io.write(b"Enter a character: ")?;
                let mut byte = 0;
                self.memory.io.read(slice::from_mut(&mut byte))?;
                self.memory.io.write(slice::from_ref(&byte))?;
                self.memory.io.flush()?;
                self.registers.r0 = byte as u16;
            }
            TrapCode::PUTSP => unsafe {
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
            TrapCode::HALT => {
                self.memory.io.write(b"HALT\n")?;
                return Ok(Status::Halt);
            }
        }
        Ok(Status::Continue)
    }
}

#[must_use]
pub enum Status {
    Continue,
    Halt,
}

pub enum Error<IO> {
    Io(IO),
    OpCodeNotImplemented,
    SystemCallNotImplemented,
}

impl<IO> From<IO> for Error<IO> {
    fn from(value: IO) -> Self {
        Error::Io(value)
    }
}
