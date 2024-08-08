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
/// - Arithmetics: [`OpCode::ADD`], [`OpCode::AND`], [`OpCode::NOT`]
/// - Control flow: [`OpCode::BR`], [`OpCode::JMP`], [`OpCode::JSR`], [`OpCode::RTI`]
/// - Load data: [`OpCode::LD`], [`OpCode::LDI`], [`OpCode::LDR`], [`OpCode::LEA`]
/// - Store data: [`OpCode::ST`], [`OpCode::STR`], [`OpCode::STI`]
/// - Input/Output: [`OpCode::TRAP`]
/// - Reserved: [`OpCode::RES`]
///
pub enum OpCode {
    /// Conditional Branch.
    BR,
    /// Addition.
    ADD,
    /// Load.
    LD,
    /// Store.
    ST,
    /// Jump to subroutine.
    #[doc(alias = "JSRR")]
    JSR,
    /// Bitwise Logical AND.
    AND,
    /// Load Base+Offset.
    LDR,
    /// Store Base+Offset.
    STR,
    /// Return from Interrupt.
    RTI,
    /// Bitwise Complement.
    NOT,
    /// Load Indirect.
    LDI,
    /// Store Indirect.
    STI,
    /// Jump, or Return from Subroutine.
    #[doc(alias = "RET")]
    JMP,
    /// Reserved.
    RES,
    /// Load Effective Address.
    LEA,
    /// System Call.
    TRAP,
}
