extern crate rprompt;

use std::io::stdin;

fn main() {
    let stdin = stdin();

    // Prompt for a reply on TTY
    let reply = rprompt::prompt_reply(&mut stdin.lock(), "What's your name? ").unwrap();
    println!("Your reply is {}", reply);

    // Or print and read the reply separately
    rprompt::print_tty("What's your name? ").unwrap();
    let reply = rprompt::read_reply(&mut stdin.lock()).unwrap();
    println!("Your reply is {}", reply);
}
