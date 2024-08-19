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

/// Instruction opcodes in LC-3.
///
/// - Arithmetics: [`OpCode::Add`], [`OpCode::And`], [`OpCode::Not`]
/// - Control flow: [`OpCode::Br`], [`OpCode::Jmp`], [`OpCode::Jsr`], [`OpCode::Rti`]
/// - Load data: [`OpCode::Ld`], [`OpCode::Ldi`], [`OpCode::Ldr`], [`OpCode::Lea`]
/// - Store data: [`OpCode::St`], [`OpCode::Str`], [`OpCode::Sti`]
/// - Input/Output: [`OpCode::Trap`]
/// - Reserved: [`OpCode::Res`]
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OpCode {
    /// Conditional Branch.
    Br,
    /// Addition.
    Add,
    /// Load.
    Ld,
    /// Store.
    St,
    /// Jump to subroutine.
    #[doc(alias = "JSRR")]
    Jsr,
    /// Bitwise Logical AND.
    And,
    /// Load Base+Offset.
    Ldr,
    /// Store Base+Offset.
    Str,
    /// Return from Interrupt.
    Rti,
    /// Bitwise Complement.
    Not,
    /// Load Indirect.
    Ldi,
    /// Store Indirect.
    Sti,
    /// Jump, or Return from Subroutine.
    #[doc(alias = "RET")]
    Jmp,
    /// Reserved.
    Res,
    /// Load Effective Address.
    Lea,
    /// System Call.
    Trap,
}
