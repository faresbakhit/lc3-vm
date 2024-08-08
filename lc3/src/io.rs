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

/// An input device interface, analogous to [`std::io::Read`].
///
/// With the `std` feature enabled, all types that implement
/// [`std::io::Read`] also implement [`InputDevice`].
pub trait InputDevice {
    type Error;
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error>;
    fn poll(&self) -> bool {
        true
    }
}

/// An output device interface, analogous to [`std::io::Write`].
///
/// With the `std` feature enabled, all types that implement
/// [`std::io::Write`] also implement [`OutputDevice`].
pub trait OutputDevice {
    type Error;
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error>;
    fn flush(&mut self) -> Result<(), Self::Error>;
}

/// An input/output device interface, analogous to [`std::io::Read`] + [`std::io::Write`].
///
/// With the `std` feature enabled, all types that implement
/// [`std::io::Read`] + [`std::io::Write`] also implement [`IoDevice`].
pub trait IoDevice {
    type Error;
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error>;
    fn poll(&self) -> bool {
        true
    }
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error>;
    fn flush(&mut self) -> Result<(), Self::Error>;
}

#[cfg(feature = "std")]
impl<T> InputDevice for T
where
    T: std::io::Read,
{
    type Error = std::io::Error;

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        std::io::Read::read(self, buf)
    }
}

#[cfg(feature = "std")]
impl<T> OutputDevice for T
where
    T: std::io::Write,
{
    type Error = std::io::Error;

    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        std::io::Write::write(self, buf)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        std::io::Write::flush(self)
    }
}
