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

use std::io::{self, Read, Stdin, Stdout, Write};
use std::os::fd::AsRawFd;

use crate::IoDevice;

/// [termios(3)] [`IoDevice`][`crate::IoDevice`].
///
/// [termios(3)]: https://man7.org/linux/man-pages/man3/termios.3.html
pub struct Termios<W: Write + AsRawFd = Stdout, R: Read = Stdin> {
    output: W,
    input: R,
}

impl Termios {
    pub fn new() -> io::Result<Termios> {
        Termios::from(io::stdout(), io::stdin())
    }
}

impl<W: Write + AsRawFd, R: Read> Termios<W, R> {
    pub fn from(output: W, input: R) -> io::Result<Termios<W, R>> {
        let fd = output.as_raw_fd();
        let mut ios = termios::Termios::from_fd(fd)?;
        ios.c_lflag &= !termios::ICANON & !termios::ECHO;
        termios::tcsetattr(fd, termios::TCSAFLUSH, &ios)?;
        Ok(Termios { output, input })
    }
}

impl<W: Write + AsRawFd, R: Read> Drop for Termios<W, R> {
    fn drop(&mut self) {
        let mut ios = match termios::Termios::from_fd(self.output.as_raw_fd()) {
            Ok(ios) => ios,
            Err(_) => return,
        };
        ios.c_lflag |= termios::ICANON | termios::ECHO;
        let _ = termios::tcsetattr(self.output.as_raw_fd(), termios::TCSANOW, &ios);
    }
}

impl<W: Write + AsRawFd, R: Read> Read for Termios<W, R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.input.read(buf)
    }
}

impl<W: Write + AsRawFd, R: Read> Write for Termios<W, R> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.output.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.output.flush()
    }
}

impl<W: Write + AsRawFd> IoDevice for Termios<W, Stdin> {
    type Error = io::Error;

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.input.read(buf)
    }

    fn poll(&self) -> bool {
        unsafe {
            let mut readfds = std::mem::MaybeUninit::<libc::fd_set>::uninit();
            libc::FD_ZERO(readfds.as_mut_ptr());
            libc::FD_SET(libc::STDIN_FILENO, readfds.as_mut_ptr());
            let mut timeout = libc::timeval {
                tv_sec: 0,
                tv_usec: 0,
            };
            libc::select(
                1,
                readfds.as_mut_ptr(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut timeout as *mut libc::timeval,
            ) != 0
        }
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.output.write(buf)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.output.flush()
    }
}
