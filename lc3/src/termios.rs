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
use std::mem::{self, MaybeUninit};
use std::os::fd::{AsFd, AsRawFd, BorrowedFd};

use crate::IoDevice;

/// [termios(3)] [`IoDevice`][`crate::IoDevice`].
///
/// [termios(3)]: https://man7.org/linux/man-pages/man3/termios.3.html
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Termios<W: Write + AsFd = Stdout, R: Read = Stdin> {
    prev_ios: libc::termios,
    output: W,
    input: R,
}

impl Termios {
    pub fn new() -> io::Result<Termios> {
        Termios::from(io::stdout(), io::stdin())
    }
}

impl<W: Write + AsFd, R: Read> Termios<W, R> {
    pub fn from(output: W, input: R) -> io::Result<Termios<W, R>> {
        let mut ios = get_terminal_attr(output.as_fd())?;
        let prev_ios = ios;
        ios.c_lflag &= !libc::ICANON & !libc::ECHO;
        set_terminal_attr(output.as_fd(), libc::TCSAFLUSH, &ios)?;
        Ok(Termios {
            prev_ios,
            output,
            input,
        })
    }
}

impl<W: Write + AsFd, R: Read> Drop for Termios<W, R> {
    fn drop(&mut self) {
        let _ = set_terminal_attr(self.output.as_fd(), libc::TCSANOW, &self.prev_ios);
    }
}

impl<W: Write + AsFd, R: Read> Read for Termios<W, R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.input.read(buf)
    }
}

impl<W: Write + AsFd, R: Read> Write for Termios<W, R> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.output.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.output.flush()
    }
}

impl<W: Write + AsFd> IoDevice for Termios<W, Stdin> {
    type Error = io::Error;

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.input.read(buf)
    }

    fn poll(&self) -> bool {
        unsafe {
            let mut readfds = MaybeUninit::<libc::fd_set>::uninit();
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

fn get_terminal_attr(fd: BorrowedFd) -> io::Result<libc::termios> {
    unsafe {
        let mut termios = mem::zeroed();
        ret(libc::tcgetattr(fd.as_raw_fd(), &mut termios))?;
        Ok(termios)
    }
}

fn set_terminal_attr(fd: BorrowedFd, when: libc::c_int, termios: &libc::termios) -> io::Result<()> {
    ret(unsafe { libc::tcsetattr(fd.as_raw_fd(), when, termios) })
}

fn ret(raw: libc::c_int) -> io::Result<()> {
    if raw == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}
