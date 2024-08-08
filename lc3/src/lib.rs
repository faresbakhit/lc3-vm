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

//! [Little Computer 3] implementation. [Specification].
//!
//! [Little Computer 3]: https://en.wikipedia.org/wiki/Little_Computer_3
//! [Specification]: https://www.jmeiners.com/lc3-vm/supplies/lc3-isa.pdf

#![cfg_attr(not(feature = "std"), no_std)]

mod condcodes;
mod decode;
mod io;
mod lc3;
mod memory;
mod opcode;
mod registers;
#[cfg(feature = "termios")]
mod termios;
mod trapcode;

pub use condcodes::CondCodes;
pub use decode::InstructionDecode;
pub use io::{InputDevice, IoDevice, OutputDevice};
pub use lc3::{Error, LC3};
pub use memory::Memory;
pub use opcode::OpCode;
pub use registers::{Registers, GPR};
#[cfg(feature = "termios")]
pub use termios::Termios;
pub use trapcode::TrapCode;
