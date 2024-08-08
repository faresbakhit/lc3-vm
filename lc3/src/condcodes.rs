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

/// Condition codes: N (negative), Z (zero), and P (positive).
#[derive(Clone, Copy)]
pub struct CondCodes(u16);

impl CondCodes {
    /// N = 1, Z, P = 0.
    pub const N: CondCodes = CondCodes(1 << 2);
    /// Z = 1, N, P = 0.
    pub const Z: CondCodes = CondCodes(1 << 1);
    /// P = 1, N, Z = 0.
    pub const P: CondCodes = CondCodes(1 << 0);

    /// N = Z = P = 0.
    ///
    /// ```
    /// # use lc3::CondCodes;
    /// let cc = CondCodes::none();
    /// assert!(!cc.negative());
    /// assert!(!cc.zero());
    /// assert!(!cc.positive());
    /// ```
    pub const fn none() -> CondCodes {
        CondCodes(0)
    }

    /// N, Z, P ?= 0.
    ///
    /// ```
    /// # use lc3::CondCodes;
    /// assert!(CondCodes::none().is_none());
    /// assert!(!CondCodes::N.is_none());
    /// assert!(!CondCodes::Z.union(&CondCodes::P).is_none());
    /// ```
    pub const fn is_none(self) -> bool {
        self.0 == 0
    }

    /// N ?= 1.
    ///
    /// ```
    /// # use lc3::CondCodes;
    /// let cc = CondCodes::N;
    /// assert!(cc.negative());
    /// assert!(!cc.zero());
    /// assert!(!cc.positive());
    /// ```
    pub const fn negative(&self) -> bool {
        self.intersects(&CondCodes::N)
    }

    /// Z ?= 1.
    ///
    /// ```
    /// # use lc3::CondCodes;
    /// let cc = CondCodes::Z;
    /// assert!(!cc.negative());
    /// assert!(cc.zero());
    /// assert!(!cc.positive());
    /// ```
    pub const fn zero(&self) -> bool {
        self.intersects(&CondCodes::Z)
    }

    /// P ?= 1.
    ///
    /// ```
    /// # use lc3::CondCodes;
    /// let cc = CondCodes::P;
    /// assert!(!cc.negative());
    /// assert!(!cc.zero());
    /// assert!(cc.positive());
    /// ```
    pub const fn positive(&self) -> bool {
        self.intersects(&CondCodes::P)
    }

    /// [`CondCodes`] from bits \[3:0\] of a 16-bit value.
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

    /// N = N1 OR N2, Z = Z1 OR Z2 P = P1 OR P2.
    ///
    /// ```
    /// # use lc3::CondCodes;
    /// let cc1 = CondCodes::N.union(&CondCodes::Z);
    /// let cc2 = CondCodes::Z.union(&CondCodes::P);
    /// let cc3 = cc1.union(&cc2);
    /// assert!(cc3.negative());
    /// assert!(cc3.zero());
    /// assert!(cc3.positive());
    /// ```
    pub const fn union(&self, other: &CondCodes) -> CondCodes {
        CondCodes(self.0 | other.0)
    }

    /// N = N1 AND N2, Z = Z1 AND Z2 P = P1 AND P2.
    ///
    /// ```
    /// # use lc3::CondCodes;
    /// let cc1 = CondCodes::N.union(&CondCodes::Z);
    /// let cc2 = CondCodes::Z.union(&CondCodes::P);
    /// let cc3 = cc1.intersection(&cc2);
    /// assert!(!cc3.negative());
    /// assert!(cc3.zero());
    /// assert!(!cc3.positive());
    /// ```
    pub const fn intersection(&self, other: &CondCodes) -> CondCodes {
        CondCodes(self.0 & other.0)
    }

    /// (N1 AND N2) OR (Z1 AND Z2) OR (P1 AND P2).
    ///
    /// ```
    /// # use lc3::CondCodes;
    /// assert!(CondCodes::P.union(&CondCodes::N).intersects(&CondCodes::N));
    /// assert!(!CondCodes::N.union(&CondCodes::Z).intersects(&CondCodes::P));
    /// ```
    pub const fn intersects(self, other: &CondCodes) -> bool {
        !self.intersection(other).is_none()
    }
}
