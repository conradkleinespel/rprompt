# Rustastic Prompt

![CI](https://github.com/conradkleinespel/rprompt/workflows/CI/badge.svg)
[![Build status](https://ci.appveyor.com/api/projects/status/ch4ljnrsot9sk0g8?svg=true)](https://ci.appveyor.com/project/conradkleinespel/rprompt)

`rprompt` allows you to easily prompt for input in a console application on Linux, BSD, OSX and Windows.

`rprompt` is made available free of charge. You can support its development through [Liberapay](https://liberapay.com/conradkleinespel/) 💪

## Usage

Add `rprompt` as a dependency in Cargo.toml:

```toml
[dependencies]
rprompt = "1.0"
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

The full API documentation is available at [https://docs.rs/rprompt](https://docs.rs/rprompt).

## Contributors

We welcome contribution from everyone. Feel free to open an issue or a pull request at any time.

Here's a list of existing `rprompt` contributors:

* [@conradkleinespel](https://github.com/conradkleinespel)
* [@dpc](https://github.com/dpc)
* [@steveatinfincia](https://github.com/steveatinfincia)

Thank you very much for your help!  :smiley:  :heart:

## License

The source code is released under the Apache 2.0 license.
