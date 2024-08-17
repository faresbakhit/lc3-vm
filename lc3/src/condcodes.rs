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

use core::fmt;

/// Condition codes: N (negative), Z (zero), and P (positive).
#[derive(Clone, Copy)]
pub struct CondCodes(u16);

impl fmt::Debug for CondCodes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CondCodes(N = {}, Z = {}, P = {})",
            self.negative() as u16,
            self.zero() as u16,
            self.positive() as u16,
        )
    }
}

impl CondCodes {
    /// N = Z = P = 0.
    ///
    /// # Examples
    ///
    /// ```
    /// # use lc3::CondCodes;
    /// assert!(CondCodes::NONE.none());
    /// assert!(!CondCodes::NONE.any());
    /// assert!(!CondCodes::NONE.all());
    /// assert!(!CondCodes::NONE.intersects(CondCodes::ALL));
    /// ```
    pub const NONE: CondCodes = CondCodes(0);

    /// N = 1, Z, P = 0.
    ///
    /// # Examples
    ///
    /// ```
    /// # use lc3::CondCodes;
    /// assert!(CondCodes::N.negative());
    /// assert!(!CondCodes::N.zero());
    /// assert!(!CondCodes::N.positive());
    /// assert!(!CondCodes::N.none());
    /// assert!(CondCodes::N.any());
    /// assert!(!CondCodes::N.all());
    /// ```
    pub const N: CondCodes = CondCodes(1 << 2);

    /// Z = 1, N, P = 0.
    ///
    /// # Examples
    ///
    /// ```
    /// # use lc3::CondCodes;
    /// assert!(!CondCodes::Z.negative());
    /// assert!(CondCodes::Z.zero());
    /// assert!(!CondCodes::Z.positive());
    /// assert!(!CondCodes::Z.none());
    /// assert!(CondCodes::Z.any());
    /// assert!(!CondCodes::Z.all());
    /// ```
    pub const Z: CondCodes = CondCodes(1 << 1);

    /// P = 1, N, Z = 0.
    ///
    /// # Examples
    ///
    /// ```
    /// # use lc3::CondCodes;
    /// assert!(!CondCodes::P.negative());
    /// assert!(!CondCodes::P.zero());
    /// assert!(CondCodes::P.positive());
    /// assert!(!CondCodes::P.none());
    /// assert!(CondCodes::P.any());
    /// assert!(!CondCodes::P.all());
    /// ```
    pub const P: CondCodes = CondCodes(1 << 0);

    /// N = Z = P = 1.
    ///
    /// # Examples
    ///
    /// ```
    /// # use lc3::CondCodes;
    /// assert!(!CondCodes::ALL.none());
    /// assert!(CondCodes::ALL.any());
    /// assert!(CondCodes::ALL.all());
    /// assert!(!CondCodes::ALL.intersects(CondCodes::NONE));
    /// ```
    pub const ALL: CondCodes = CondCodes(0b111);

    /// N = (N1 OR N2), Z = (Z1 OR Z2), P = (P1 OR P2).
    ///
    /// # Examples
    ///
    /// ```
    /// # use lc3::CondCodes;
    /// let cc1 = CondCodes::N.union(CondCodes::Z);
    /// let cc2 = CondCodes::Z.union(CondCodes::P);
    /// let cc3 = cc1.union(cc2);
    /// assert!(cc3.negative());
    /// assert!(cc3.zero());
    /// assert!(cc3.positive());
    /// ```
    pub const fn union(self, other: CondCodes) -> CondCodes {
        CondCodes(self.0 | other.0)
    }

    /// N = (N1 AND N2), Z = (Z1 AND Z2), P = (P1 AND P2).
    ///
    /// # Examples
    ///
    /// ```
    /// # use lc3::CondCodes;
    /// let cc1 = CondCodes::N.union(CondCodes::Z);
    /// let cc2 = CondCodes::Z.union(CondCodes::P);
    /// let cc3 = cc1.intersection(cc2);
    /// assert!(!cc3.negative());
    /// assert!(cc3.zero());
    /// assert!(!cc3.positive());
    /// ```
    pub const fn intersection(self, other: CondCodes) -> CondCodes {
        CondCodes(self.0 & other.0)
    }

    /// [`CondCodes`] from bits \[3:0\] of a 16-bit value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use lc3::CondCodes;
    /// let cc = CondCodes::from_u16(0b110);
    /// assert!(cc.negative());
    /// assert!(cc.zero());
    /// assert!(!cc.positive());
    /// ```
    pub const fn from_u16(value: u16) -> CondCodes {
        CondCodes(value & 0x7)
    }

    /// [`CondCodes`] from signedness of number.
    ///
    /// # Examples
    ///
    /// ```
    /// # use lc3::CondCodes;
    /// let cc = CondCodes::from_signum(0x8001);
    /// assert!(cc.negative());
    /// assert!(!cc.zero());
    /// assert!(!cc.positive());
    /// ```
    pub const fn from_signum(num: u16) -> CondCodes {
        if num == 0 {
            CondCodes::Z
        } else if (num >> 15) != 0 {
            CondCodes::N
        } else {
            CondCodes::P
        }
    }

    /// N, Z, P ?= 0.
    ///
    /// # Examples
    ///
    /// ```
    /// # use lc3::CondCodes;
    /// assert!(CondCodes::NONE.none());
    /// assert!(!CondCodes::N.none());
    /// assert!(!CondCodes::Z.union(CondCodes::P).none());
    /// ```
    pub const fn none(self) -> bool {
        self.is(CondCodes::NONE)
    }

    /// N ?= 1.
    ///
    /// # Examples
    ///
    /// ```
    /// # use lc3::CondCodes;
    /// let cc = CondCodes::N;
    /// assert!(cc.negative());
    /// assert!(!cc.zero());
    /// assert!(!cc.positive());
    /// ```
    pub const fn negative(self) -> bool {
        self.intersects(CondCodes::N)
    }

    /// Z ?= 1.
    ///
    /// # Examples
    ///
    /// ```
    /// # use lc3::CondCodes;
    /// let cc = CondCodes::Z;
    /// assert!(!cc.negative());
    /// assert!(cc.zero());
    /// assert!(!cc.positive());
    /// ```
    pub const fn zero(self) -> bool {
        self.intersects(CondCodes::Z)
    }

    /// P ?= 1.
    ///
    /// # Examples
    ///
    /// ```
    /// # use lc3::CondCodes;
    /// let cc = CondCodes::P;
    /// assert!(!cc.negative());
    /// assert!(!cc.zero());
    /// assert!(cc.positive());
    /// ```
    pub const fn positive(self) -> bool {
        self.intersects(CondCodes::P)
    }

    /// N = 1 OR Z = 1 OR P = 1.
    ///
    /// # Examples
    ///
    /// ```
    /// # use lc3::CondCodes;
    /// assert!(CondCodes::N.any());
    /// assert!(CondCodes::P.union(CondCodes::Z).any());
    /// assert!(!CondCodes::NONE.any());
    /// ```
    pub const fn any(self) -> bool {
        !self.none()
    }

    /// N, Z, P ?= 1
    ///
    /// # Examples
    ///
    /// ```
    /// # use lc3::CondCodes;
    /// assert!(CondCodes::ALL.all());
    /// assert!(!CondCodes::NONE.all());
    /// assert!(!CondCodes::N.union(CondCodes::P).all());
    /// ```
    pub const fn all(self) -> bool {
        self.is(CondCodes::ALL)
    }

    /// N1 ?= N2 AND Z1 ?= Z2 AND P1 ?= P2.
    ///
    /// # Examples
    ///
    /// ```
    /// # use lc3::CondCodes;
    /// assert!(!CondCodes::N.union(CondCodes::Z).is(CondCodes::Z));
    /// assert!(CondCodes::P.is(CondCodes::P));
    /// ```
    pub const fn is(self, other: CondCodes) -> bool {
        self.0 == other.0
    }

    /// (N1 AND N2) OR (Z1 AND Z2) OR (P1 AND P2).
    ///
    /// # Examples
    ///
    /// ```
    /// # use lc3::CondCodes;
    /// assert!(CondCodes::P.union(CondCodes::N).intersects(CondCodes::N));
    /// assert!(!CondCodes::N.union(CondCodes::Z).intersects(CondCodes::P));
    /// ```
    pub const fn intersects(self, other: CondCodes) -> bool {
        self.intersection(other).any()
    }
}
