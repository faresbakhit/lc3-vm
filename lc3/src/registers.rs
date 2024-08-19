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

use crate::CondCodes;
use core::ops::{Index, IndexMut};

/// General purpose registers of LC-3.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Reg {
    /// First general-purpose register.
    R0,
    /// Second general-purpose register.
    R1,
    /// Third general-purpose register.
    R2,
    /// Fourth general-purpose register.
    R3,
    /// Fifth general-purpose register.
    R4,
    /// Sixth general-purpose register.
    R5,
    /// Seventh general-purpose register.
    R6,
    /// Eighth and last general-purpose register.
    R7,
}

impl Reg {
    /// [`Reg`] from bits \[3:0\] of a 16-bit value.
    pub const fn from_u16(value: u16) -> Reg {
        unsafe { Reg::from_u16_unchecked(value & 0x7) }
    }

    /// [`Reg`] from a 16-bit value (bounds unchecked).
    pub const unsafe fn from_u16_unchecked(value: u16) -> Reg {
        core::mem::transmute(value as i8)
    }
}

/// Registers of LC-3, indexable by [`Reg`].
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, Default)]
#[repr(C)]
pub struct Registers {
    /// First general-purpose register.
    pub r0: u16,
    /// Second general-purpose register.
    pub r1: u16,
    /// Third general-purpose register.
    pub r2: u16,
    /// Fourth general-purpose register.
    pub r3: u16,
    /// Fifth general-purpose register.
    pub r4: u16,
    /// Sixth general-purpose register.
    pub r5: u16,
    /// Seventh general-purpose register.
    pub r6: u16,
    /// Eighth and last general-purpose register.
    pub r7: u16,
    /// Program counter register.
    pub pc: u16,
    /// Condition codes registers.
    pub cc: CondCodes,
}

impl Registers {
    /// Initalize a new registers unit.
    pub const fn new() -> Registers {
        Registers {
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            r5: 0,
            r6: 0,
            r7: 0,
            pc: 0,
            cc: CondCodes::NONE,
        }
    }
}

impl Index<Reg> for Registers {
    type Output = u16;

    fn index(&self, index: Reg) -> &u16 {
        unsafe { &*(self as *const Registers as *mut u16).add(index as usize) }
    }
}

impl IndexMut<Reg> for Registers {
    fn index_mut(&mut self, index: Reg) -> &mut u16 {
        unsafe { &mut *(self as *mut Registers as *mut u16).add(index as usize) }
    }
}

/// I/O device registers for memory-mapped I/O.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum IoDeviceRegister {
    /// Keyboard status register.
    Kbsr = 0xFE00,
    /// Keyboard data register.
    Kbdr = 0xFE02,
    /// Display status register.
    Dsr = 0xFE04,
    /// Display data register.
    Ddr = 0xFE06,
    /// Machine control register.
    Mcr = 0xFFFE,
}

impl IoDeviceRegister {
    pub const STATUS_ACCEPT: u16 = 0x8000;
    pub const STATUS_DECLINE: u16 = 0;

    pub const fn from_u16(value: u16) -> Option<IoDeviceRegister> {
        match value {
            0xFE00 => Some(IoDeviceRegister::Kbsr),
            0xFE02 => Some(IoDeviceRegister::Kbdr),
            0xFE04 => Some(IoDeviceRegister::Dsr),
            0xFE06 => Some(IoDeviceRegister::Ddr),
            0xFFFE => Some(IoDeviceRegister::Mcr),
            _ => None,
        }
    }
}
