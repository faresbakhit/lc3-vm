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

use crate::{IoDevice, IoDeviceRegister};

/// Number of 'words' in [`Memory`] or length of underlying slice.
const LEN: usize = 1 << 16;

/// Main memory unit in LC-3.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Memory<IO: IoDevice> {
    words: [u16; LEN],
    pub(crate) io: IO,
}

impl<IO: IoDevice + Default> Default for Memory<IO> {
    fn default() -> Memory<IO> {
        Memory {
            words: [0; LEN],
            io: Default::default(),
        }
    }
}

impl<IO: IoDevice> Memory<IO> {
    /// Initialize a new memory device.
    pub const fn new(iodevice: IO) -> Memory<IO> {
        Memory {
            words: [0; LEN],
            io: iodevice,
        }
    }

    /// Read the value at index `index` in memory.
    pub fn read(&mut self, index: u16) -> u16 {
        match IoDeviceRegister::from_u16(index) {
            Some(IoDeviceRegister::Kbsr) => {
                if self.io.poll() {
                    IoDeviceRegister::STATUS_ACCEPT
                } else {
                    IoDeviceRegister::STATUS_DECLINE
                }
            }
            Some(IoDeviceRegister::Kbdr) => {
                if self.io.poll() {
                    let mut byte = 0;
                    let _ = self.io.read(slice::from_mut(&mut byte));
                    byte as u16
                } else {
                    IoDeviceRegister::STATUS_DECLINE
                }
            }
            Some(IoDeviceRegister::Dsr) => IoDeviceRegister::STATUS_ACCEPT,
            Some(IoDeviceRegister::Ddr) => IoDeviceRegister::STATUS_DECLINE,
            _ => self.words[index as usize],
        }
    }

    /// Write `value` to the index `index` in memory.
    pub fn write(&mut self, index: u16, value: u16) {
        match IoDeviceRegister::from_u16(index) {
            Some(IoDeviceRegister::Mcr) | None => {
                self.words[index as usize] = value;
            }
            Some(IoDeviceRegister::Ddr) => {
                let byte = value as u8;
                let _ = self.io.write(slice::from_ref(&byte));
                let _ = self.io.flush();
                return;
            }
            _ => return,
        }
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
