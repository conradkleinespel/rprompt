//! This library makes it easy to prompt for input in a console application on all platforms, Unix
//! and Windows alike.
//!
//! Here's how you can prompt for a reply:
//! ```no_run
//! let name = rprompt::prompt_reply("What's your name? ").unwrap();
//! println!("Your name is {}", name);
//! ```
//!
//! Alternatively, you can read the reply without prompting:
//! ```no_run
//! let name = rprompt::read_reply().unwrap();
//! println!("Your name is {}", name);
//! ```
//!
//! If you need more control over the source of the input, which can be useful if you want to unit
//! test your CLI or handle pipes gracefully, you can use `from_bufread` versions of the functions
//! and pass any reader you want:
//! ```no_run
//! let stdin = std::io::stdin();
//! let stdout = std::io::stdout();
//! let name = rprompt::prompt_reply_from_bufread(&mut stdin.lock(), &mut stdout.lock(), "What's your name? ").unwrap();
//! println!("Your name is {}", name);
//! ```

mod rprompt;

pub use crate::rprompt::*;
