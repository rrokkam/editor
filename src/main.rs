use std::io::{self, Read, Write};
use termion::raw::IntoRawMode;

fn main() {
    let mut stdout = io::stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        let c = b.unwrap() as char;
        println!("{}\r", c);
        if c == 'q' {
            break;
        }
    }
    writeln!(stdout, "Hello, world!").unwrap();
}
