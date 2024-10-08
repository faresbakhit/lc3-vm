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

use lc3::{self, Lc3, Termios};
use std::{env, fmt, fs::File, io, path::PathBuf, process::ExitCode};

const LICENSE: &str = "lc3-vm  Copyright (c) 2024  Fares A. Bakhit <fares@duck.com>";
const USAGE: &str = "[--no-default-os] [--virtual-trap-vector-table] [IMAGE-FILE...]";

fn main() -> ExitCode {
    let arg0 = env::args().next().unwrap_or("path/to/lc3-vm".into());

    if env::args().len() <= 1 {
        eprintln!("{LICENSE}");
        eprintln!("Usage: {arg0} {USAGE}");
        return ExitCode::from(2);
    }

    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{arg0}: {err}");
            match err.kind {
                ErrorKind::Io(_) => ExitCode::from(2),
                ErrorKind::Lc3(_) => ExitCode::FAILURE,
                ErrorKind::UnrecognizedOption => {
                    eprintln!("Usage: {arg0} {USAGE}");
                    ExitCode::from(2)
                }
            }
        }
    }
}

fn run() -> Result<(), Error> {
    let mut files = Vec::with_capacity(env::args_os().len());
    let mut default_os = true;
    let mut virtual_trap_vector_table = false;
    let mut stop_options_processing = false;

    for arg in env::args_os().skip(1) {
        if stop_options_processing {
            files.push(arg);
        } else if arg == "--no-default-os" {
            default_os = false;
        } else if arg == "--virtual-trap-vector-table" {
            virtual_trap_vector_table = true;
        } else if arg == "--" {
            stop_options_processing = true;
        } else if arg.as_encoded_bytes().starts_with(b"-") {
            return Err(Error::new(
                ErrorKind::UnrecognizedOption,
                PathBuf::from(arg).display(),
            ));
        } else {
            files.push(arg);
        }
    }

    let mut lc3 = Lc3::new(Termios::new()?);

    if default_os {
        let lc3os_img = include_bytes!("lc3os.obj");
        lc3.load_image(&mut lc3os_img.as_slice())?;
    }

    files.into_iter().try_for_each(|x| {
        File::open(&x)
            .and_then(|mut x| lc3.load_image(&mut x))
            .err_with_context(PathBuf::from(x).display())
    })?;

    if virtual_trap_vector_table {
        lc3.run_with_virtual_trap_vector_table()
    } else {
        lc3.run()
    }
    .err_with_context("<termios>")
}

struct Error {
    kind: ErrorKind,
    ctx: String,
}

impl Error {
    fn new<C: ToString>(err: ErrorKind, ctx: C) -> Error {
        Error {
            kind: err,
            ctx: ctx.to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let ErrorKind::UnrecognizedOption = self.kind {
            write!(f, "{} '{}'", self.kind, self.ctx)
        } else {
            write!(f, "{}: {}", self.ctx, self.kind)
        }
    }
}

trait ErrWithContext<T, E> {
    fn err_with_context<C: ToString>(self, ctx: C) -> Result<T, Error>;
}

impl<T, E> ErrWithContext<T, E> for Result<T, E>
where
    ErrorKind: From<E>,
{
    fn err_with_context<C: ToString>(self, ctx: C) -> Result<T, Error> {
        self.map_err(|err| Error::new(ErrorKind::from(err), ctx))
    }
}

enum ErrorKind {
    Io(io::Error),
    Lc3(lc3::Error<io::Error>),
    UnrecognizedOption,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorKind::Io(err) => err.fmt(f),
            ErrorKind::Lc3(err) => err.fmt(f),
            Self::UnrecognizedOption => f.write_str("unrecognized option"),
        }
    }
}

impl From<io::Error> for ErrorKind {
    fn from(value: io::Error) -> ErrorKind {
        ErrorKind::Io(value)
    }
}

impl From<lc3::Error<io::Error>> for ErrorKind {
    fn from(value: lc3::Error<io::Error>) -> ErrorKind {
        ErrorKind::Lc3(value)
    }
}

impl<E> From<E> for Error
where
    ErrorKind: From<E>,
{
    fn from(value: E) -> Error {
        Error {
            kind: ErrorKind::from(value),
            ctx: String::new(),
        }
    }
}
