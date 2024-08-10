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

use lc3::{Error, Termios, LC3};
use std::{env, fs::File, io, process::ExitCode};

const LICENSE: &str = "lc3-vm  Copyright (c) 2024  Fares A. Bakhit <fares@duck.com>";

fn main() -> ExitCode {
    if env::args().len() <= 1 {
        let arg0 = env::args().next().unwrap_or("path/to/lc3-vm".into());
        eprintln!("{}", LICENSE);
        eprintln!("Usage: {} [IMAGE-FILE...]", arg0);
        return ExitCode::from(2);
    }

    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(Error::Io(err)) => {
            eprintln!("Error: {}", err);
            ExitCode::from(2)
        }
        Err(_) => ExitCode::FAILURE,
    }
}

fn run() -> Result<(), Error<io::Error>> {
    let mut lc3 = LC3::new(Termios::new()?);

    for arg in env::args().skip(1) {
        let mut file = File::open(arg)?;
        lc3.load_image(&mut file)?;
    }

    lc3.run()
}
