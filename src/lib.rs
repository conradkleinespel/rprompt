//! This library makes it easy to prompt for input in a console application on all platforms, Unix
//! and Windows alike.
//!
//! Here's how you can prompt for a reply:
//! ```no_run
//! let name = rprompt::prompt_reply("What's your name? ").unwrap();
//! println!("Your name is {}", name);
//! ```
//!
//! Alternatively, you can print and ask for input separately:
//! ```no_run
//! rprompt::print_tty("What's your name? ").unwrap();
//! let name = rprompt::read_reply().unwrap();
//! println!("Your name is {}", name);
//! ```
//!
//! If you need more control over the source of the input, which can be useful if you want to unit
//! test your CLI or handle pipes gracefully, you can use `from_bufread` versions of the functions
//! and pass any reader you want:
//! ```no_run
//! let stdin = std::io::stdin();
//! let name = rprompt::prompt_reply_from_bufread(&mut stdin.lock(), "What's your name? ").unwrap();
//! println!("Your name is {}", name);
//! ```

#[cfg(windows)]
extern crate winapi;

extern crate rutil;

/// Reads user input
pub fn read_reply(reader: &mut impl BufRead) -> std::io::Result<String> {
    let mut reply = String::new();

    reader.read_line(&mut reply)?;

    rutil::fix_new_line::fix_new_line(reply)
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
    use std::io::Write;
    use std::os::windows::io::FromRawHandle;
    use winapi::um::fileapi::{CreateFileA, OPEN_EXISTING};
    use winapi::um::handleapi::INVALID_HANDLE_VALUE;
    use winapi::um::winnt::{FILE_SHARE_READ, FILE_SHARE_WRITE, GENERIC_READ, GENERIC_WRITE};

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
