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

use crate::{CondCodes, OpCode, Reg, TrapCode};

/// Instruction decoding functions.
///
/// Instruction Encoding
/// ====================
///
/// Instructions are 16 bits wide. Bits \[15:12\] specify the [`OpCode`],
/// bits \[11:0\] provide further information that is needed to execute
/// the instruction.
///
/// ```text
///  XXXX   XXXXXXXXXXXX
/// │    │ │            │
/// └────┘ └────────────┘
/// opcode   parameters
/// ```
pub trait InstructionDecode {
    /// [`OpCode`]; bits \[15:12\].
    ///
    /// ```text
    ///  XXXX  XXXXXXXXXXXX
    /// │    │
    /// └────┘
    /// opcode
    /// ```
    fn opcode(self) -> OpCode;
    /// Condition codes; bits \[11:9\].
    ///
    /// The specification defines the condition codes as
    /// three 1-bit registers, but treating them as one
    /// 3-bit value is more efficient to.
    fn condcodes(self) -> CondCodes;
    /// Trap code; bits \[7:0\].
    ///
    /// ```text
    ///  XXXX XXXX XXXXXXXX
    ///           │        │
    ///           └────────┘
    ///            trapcode
    /// ```
    fn trapcode(self) -> Option<TrapCode>;
    /// A DR/SR register value; bits \[11:9\].
    ///
    /// ```text
    ///  XXXX XXX XXXXXXXXX
    ///      │   │
    ///      └───┘
    ///      reg1
    /// ```
    #[doc(alias = "dr")]
    #[doc(alias = "sr")]
    fn reg1(self) -> Reg;
    /// A BaseR/SR1 register value; bits \[8:6\].
    ///
    /// ```text
    ///  XXXX XXX XXX XXXXXX
    ///          │   │
    ///          └───┘
    ///          reg2
    /// ```
    #[doc(alias = "sr1")]
    #[doc(alias = "baser")]
    fn reg2(self) -> Reg;
    /// A SR2 register value; bits \[2:0\].
    ///
    /// ```text
    ///  XXXX XXXXXXXXX XXX
    ///                │   │
    ///                └───┘
    ///                reg3
    /// ```
    #[doc(alias = "sr2")]
    fn reg3(self) -> Reg;
    /// Check if the bit *b* is set to 1, starting from 0.
    fn isbitset(self, b: i32) -> bool;
    /// A 5-bit sign-extended immediate value; bits \[4:0\] of an instruction.
    ///
    /// ```text
    ///  XXXX XXXXXXX XXXXX
    ///              │     │
    ///              └─────┘
    ///               imm5
    /// ```
    fn imm5(self) -> u16;
    /// A 6-bit sign-extended value; bits \[5:0\] of an instruction.
    ///
    /// ```text
    ///  XXXX XXXXXX XXXXXX
    ///             │      │
    ///             └──────┘
    ///               imm6
    /// ```
    #[doc(alias = "offset6")]
    fn imm6(self) -> u16;
    /// An 8-bit sign-extended value; bits \[7:0\] of an instruction.
    ///
    /// ```text
    ///  XXXX XXXXX XXXXXXX
    ///            │       │
    ///            └───────┘
    ///              imm8
    /// ```
    #[doc(alias = "trapvect8")]
    fn imm8(self) -> u16;
    /// A 9-bit sign-extended value; bits \[8:0\] of an instruction.
    ///
    /// ```text
    ///  XXXX XXX XXXXXXXXX
    ///          │         │
    ///          └─────────┘
    ///             imm9
    /// ```
    #[doc(alias = "pcoffset9")]
    fn imm9(self) -> u16;
    /// An 11-bit sign-extended value; bits \[10:0\] of an instruction.
    ///
    /// ```text
    ///  XXXX X XXXXXXXXXXX
    ///        │           │
    ///        └───────────┘
    ///            imm11
    /// ```
    #[doc(alias = "pcoffset11")]
    fn imm11(self) -> u16;
}

impl InstructionDecode for u16 {
    fn opcode(self) -> OpCode {
        // SAFETY: Shifting right a 16-bit value by 12 bits
        //         leaves off 4 bits, enough to encode 2^4=16
        //         number of variants which [`OpCode`] exactly has.
        //
        // TODO: A possible improvement for additional safety:
        //
        //     debug_assert_eq!(std::mem::variant_count<OpCode>(), 1 << 4);
        //
        // but std::mem::variant_count is nightly-only as of now.
        //
        unsafe { core::mem::transmute((self >> 12) as i8) }
    }

    fn condcodes(self) -> CondCodes {
        CondCodes::from_u16(self >> 9)
    }

    fn trapcode(self) -> Option<TrapCode> {
        TrapCode::from_u16(self)
    }

    fn reg1(self) -> Reg {
        Reg::from_u16(self >> 9)
    }

    fn reg2(self) -> Reg {
        Reg::from_u16(self >> 6)
    }

    fn reg3(self) -> Reg {
        Reg::from_u16(self)
    }

    fn isbitset(self, b: i32) -> bool {
        ((self >> b) & 1) != 0
    }

    fn imm5(self) -> u16 {
        if self & 0x0010 == 0 {
            self & 0x000F
        } else {
            self | 0xFFF0
        }
    }

    fn imm6(self) -> u16 {
        if self & 0x0020 == 0 {
            self & 0x001F
        } else {
            self | 0xFFE0
        }
    }

    fn imm8(self) -> u16 {
        if self & 0x0080 == 0 {
            self & 0x00FF
        } else {
            self | 0xFF00
        }
    }

    fn imm9(self) -> u16 {
        if self & 0x0100 == 0 {
            self & 0x01FF
        } else {
            self | 0xFE00
        }
    }

    fn imm11(self) -> u16 {
        if self & 0x0400 == 0 {
            self & 0x03FF
        } else {
            self | 0xFC00
        }
    }
}
