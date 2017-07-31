# Rustastic Prompt

[![Build Status](https://travis-ci.org/conradkdotcom/rprompt.svg?branch=master)](https://travis-ci.org/conradkdotcom/rprompt)
[![Build status](https://ci.appveyor.com/api/projects/status/ch4ljnrsot9sk0g8?svg=true)](https://ci.appveyor.com/project/conradkdotcom/rprompt)

This [Rust](http://www.rust-lang.org/) package allows you to safely read
replys from standard input in a console application.

You can build the documentation with `cargo doc` or [view it online](https://docs.rs/rprompt/).

I'd appreciate feedback if you use this library :-)

## Usage

Add `rprompt` as a dependency in Cargo.toml:

```toml
[dependencies]
rprompt = "1.0.0"
```

Use `rprompt` within your code:

```rust
extern crate rprompt;

fn main() {
    // Prompt for a reply on STDOUT
    let reply = rprompt::prompt_reply_stdout("Password: ").unwrap();
    println!("Your reply is {}", reply);

    // Prompt for a reply on STDERR
    let reply = rprompt::prompt_reply_stderr("Password: ").unwrap();
    println!("Your reply is {}", reply);

    // Read a reply without prompt
    let reply = rprompt::read_reply().unwrap();
    println!("Your reply is {}", reply);
}
```

## Contributors

We welcome contribution from everyone. Feel free to open an issue or a pull request at any time.

Check out the [unassigned issues](https://github.com/conradkdotcom/rprompt/issues?q=is%3Aissue+is%3Aopen+label%3Aunassigned) to get started. If you have any questions, just let us know and we'll jump in to help.

Here's a list of existing `rprompt` contributors:

* [@conradkleinespel](https://github.com/conradkleinespel)
* [@steveatinfincia](https://github.com/steveatinfincia)

Thank you very much for your help!  :smiley:  :heart:

## Donations

`rprompt` is and will remain free for everyone. If you feel like making a donation, I appreciate it though. Here are a few ways you can donate to support `rprompt` development:
- with Bitcoin: `13SPYbEZP9kvmtsKt6C9cYj4N6Ep7my6gL`
- with Litecoin: `LN6K6fn9Mq94vbiLNzW1GZ1TvWkVihyn8T`

If you cannot afford to donate, that's OK too. Just enjoy `rprompt`! :-)
