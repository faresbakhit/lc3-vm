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

use core::ops::{Index, IndexMut};
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
}

impl<IO: IoDevice> Index<u16> for Memory<IO> {
    type Output = u16;

    fn index(&self, index: u16) -> &Self::Output {
        self.index(index as usize)
    }
}

impl<IO: IoDevice> Index<usize> for Memory<IO> {
    type Output = u16;

    #[allow(invalid_reference_casting)]
    fn index(&self, index: usize) -> &Self::Output {
        const KBSR: usize = 0xFE00;
        const KBDR: usize = 0xFE02;

        if index == KBSR {
            // Yes, I'm really doing this.
            let memory = unsafe { &mut *(self as *const _ as *mut Memory<IO>) };
            if self.io.poll() {
                memory[KBSR] = 1 << 15;
                let mut byte = 0;
                let _ = memory.io.read(slice::from_mut(&mut byte));
                memory[KBDR] = byte as u16;
            } else {
                memory[KBSR] = 0;
            }
        }

        &self.words[index]
    }
}

impl<IO: IoDevice> IndexMut<u16> for Memory<IO> {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        self.index_mut(index as usize)
    }
}

impl<IO: IoDevice> IndexMut<usize> for Memory<IO> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.words[index]
    }
}

impl<IO: IoDevice> AsRef<[u16; LEN]> for Memory<IO> {
    fn as_ref(&self) -> &[u16; LEN] {
        &self.words
    }
}

impl<IO: IoDevice> AsMut<[u16; LEN]> for Memory<IO> {
    fn as_mut(&mut self) -> &mut [u16; LEN] {
        &mut self.words
    }
}
