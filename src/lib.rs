// Copyright 2014-2017 The Rprompt Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[cfg(windows)]
extern crate winapi;

/// Reads user input
pub fn read_reply(reader: &mut impl BufRead) -> std::io::Result<String> {
    let mut reply = String::new();

    reader.read_line(&mut reply)?;

    // We should have a newline at the end. This helps prevent things such as:
    // > printf "no-newline" | program-using-rprompt
    // If we didn't have the \n check, we'd be removing the last "e" by mistake.
    if !reply.ends_with('\n') {
        return Err(std::io::Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "unexpected end of file",
        ));
    }

    // Remove the \n from the line.
    reply.pop();

    // Remove the \r from the line if present
    if reply.ends_with('\r') {
        reply.pop();
    }

    Ok(reply)
}

/// Displays a message on the TTY, then reads user input
pub fn prompt_reply(reader: &mut impl BufRead, prompt: impl ToString) -> std::io::Result<String> {
    print_tty(prompt.to_string().as_str()).and_then(|_| read_reply(reader))
}

#[cfg(unix)]
mod unix {
    use std::io::Write;

    /// Displays a message on the TTY
    pub fn print_tty(prompt: impl ToString) -> ::std::io::Result<()> {
        let mut stream = ::std::fs::OpenOptions::new().write(true).open("/dev/tty")?;
        write!(stream, "{}", prompt.to_string().as_str())?;
        stream.flush()
    }
}

#[cfg(windows)]
mod windows {
    use std::io::{self, Write};
    use std::os::windows::io::{AsRawHandle, FromRawHandle};
    use std::ptr;
    use winapi::shared::minwindef::LPDWORD;
    use winapi::um::consoleapi::{GetConsoleMode, SetConsoleMode};
    use winapi::um::fileapi::{CreateFileA, GetFileType, OPEN_EXISTING};
    use winapi::um::handleapi::INVALID_HANDLE_VALUE;
    use winapi::um::processenv::GetStdHandle;
    use winapi::um::winbase::{FILE_TYPE_PIPE, STD_INPUT_HANDLE};
    use winapi::um::wincon::{ENABLE_LINE_INPUT, ENABLE_PROCESSED_INPUT};
    use winapi::um::winnt::{
        FILE_SHARE_READ, FILE_SHARE_WRITE, GENERIC_READ, GENERIC_WRITE, HANDLE,
    };

    /// Displays a message on the TTY
    pub fn print_tty(prompt: impl ToString) -> ::std::io::Result<()> {
        let handle = unsafe {
            CreateFileA(
                b"CONOUT$\x00".as_ptr() as *const i8,
                GENERIC_READ | GENERIC_WRITE,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                ::std::ptr::null_mut(),
                OPEN_EXISTING,
                0,
                ::std::ptr::null_mut(),
            )
        };
        if handle == INVALID_HANDLE_VALUE {
            return Err(::std::io::Error::last_os_error());
        }

        let mut stream = unsafe { ::std::fs::File::from_raw_handle(handle) };

        write!(stream, "{}", prompt.to_string().as_str())?;
        stream.flush()
    }
}

use std::io::BufRead;
#[cfg(unix)]
pub use unix::print_tty;
#[cfg(windows)]
pub use windows::print_tty;
