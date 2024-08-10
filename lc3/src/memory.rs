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

use core::slice;

use crate::IoDevice;

/// Number of 'words' in [`Memory`] or length of underlying slice.
const LEN: usize = 1 << 16;

/// Main memory unit in LC-3.
pub struct Memory<IO: IoDevice> {
    words: [u16; LEN],
    pub(crate) io: IO,
}

impl<IO: IoDevice> Memory<IO> {
    /// Number of 'words' in [`Memory`] or length of underlying slice.
    pub const LEN: usize = LEN;

    /// Initialize a new memory device.
    pub const fn new(iodevice: IO) -> Memory<IO> {
        Memory {
            words: [0; LEN],
            io: iodevice,
        }
    }

    pub fn read(&mut self, index: u16) -> u16 {
        const KBSR: usize = 0xFE00;
        const KBDR: usize = 0xFE02;

        if usize::from(index) == KBSR {
            if self.io.poll() {
                self.words[KBSR] = 1 << 15;
                let mut byte = 0;
                let _ = self.io.read(slice::from_mut(&mut byte));
                self.words[KBDR] = u16::from(byte);
            } else {
                self.words[KBSR] = 0;
            }
        }

        self.words[index as usize]
    }

    pub fn write(&mut self, index: u16, value: u16) {
        self.words[index as usize] = value;
    }
}

impl<IO: IoDevice> AsRef<[u16]> for Memory<IO> {
    fn as_ref(&self) -> &[u16] {
        &self.words
    }
}

impl<IO: IoDevice> AsMut<[u16]> for Memory<IO> {
    fn as_mut(&mut self) -> &mut [u16] {
        &mut self.words
    }
}
