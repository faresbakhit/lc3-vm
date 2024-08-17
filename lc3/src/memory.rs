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
    /// Keyboard status register.
    pub const KBSR: u16 = 0xFE00;
    /// Keyboard data register.
    pub const KBDR: u16 = 0xFE02;
    /// Display status register.
    pub const DSR: u16 = 0xFE04;
    /// Display data register.
    pub const DDR: u16 = 0xFE06;
    /// Machine control register.
    pub const MCR: u16 = 0xFFFE;

    /// Initialize a new memory device.
    pub const fn new(iodevice: IO) -> Memory<IO> {
        let mut words = [0; LEN];
        words[Self::MCR as usize] = 1 << 15;
        Memory {
            words,
            io: iodevice,
        }
    }

    /// Read the value at index `index` in memory.
    pub fn read(&mut self, index: u16) -> u16 {
        match index {
            Self::KBSR => {
                if self.io.poll() {
                    1 << 15
                } else {
                    0
                }
            }
            Self::KBDR => {
                if self.io.poll() {
                    let mut byte = 0;
                    let _ = self.io.read(slice::from_mut(&mut byte));
                    byte as u16
                } else {
                    0
                }
            }
            Self::DSR => 1 << 15,
            Self::DDR => 0,
            _ => self.words[index as usize],
        }
    }

    /// Write `value` to the index `index` in memory.
    pub fn write(&mut self, index: u16, value: u16) {
        match index {
            Self::KBSR | Self::KBDR | Self::DSR => return,
            Self::DDR => {
                let byte = value as u8;
                let _ = self.io.write(slice::from_ref(&byte));
                let _ = self.io.flush();
                return;
            }
            _ => {}
        }
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
