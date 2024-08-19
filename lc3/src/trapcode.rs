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

/// [`Trap`][`crate::OpCode::Trap`] instruction opcodes.
pub enum TrapCode {
    /// Read a single character from the keyboard. The character is not echoed onto the
    /// console. Its ASCII code is copied into [`Reg::R0`][`crate::Reg::R0`]. The high eight bits of R0 are cleared.
    Getc = 0x20,
    /// Write a character in [`Reg::R0`][`crate::Reg::R0`] \[7:0\] to the console display.
    Out,
    /// Write a string of ASCII characters to the console display. The characters are contained
    /// in consecutive memory locations, one character per memory location, starting with the
    /// address specified in [`Reg::R0`][`crate::Reg::R0`]. Writing terminates with the occurrence of x0000 in a
    /// memory location.
    Puts,
    /// Print a prompt on the screen and read a single character from the keyboard. The
    /// character is echoed onto the console monitor, and its ASCII code is copied into [`Reg::R0`][`crate::Reg::R0`].
    /// The high eight bits of [`Reg::R0`][`crate::Reg::R0`] are cleared.
    In,
    /// Write a string of ASCII characters to the console. The characters are contained in
    /// consecutive memory locations, two characters per memory location, starting with the
    /// address specified in R0. The ASCII code contained in bits \[7:0\] of a memory location
    /// is written to the console first. Then the ASCII code contained in bits \[15:8\] of that
    /// memory location is written to the console. (A character string consisting of an odd
    /// number of characters to be written will have x00 in bits \[15:8\] of the memory
    /// location containing the last character to be written.) Writing terminates with the
    /// occurrence of x0000 in a memory location.
    PutSp,
    /// Halt execution and print a message on the console.
    Halt,
}

impl TrapCode {
    /// [`TrapCode`] from bits \[7:0\] of a 16-bit value.
    pub const fn from_u16(value: u16) -> Option<TrapCode> {
        match value & 0xFF {
            0x20 => Some(TrapCode::Getc),
            0x21 => Some(TrapCode::Out),
            0x22 => Some(TrapCode::Puts),
            0x23 => Some(TrapCode::In),
            0x24 => Some(TrapCode::PutSp),
            0x25 => Some(TrapCode::Halt),
            _ => None,
        }
    }
}
