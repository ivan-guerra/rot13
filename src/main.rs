use clap::Parser;
use std::io::{self, Read};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(help = "ROT13 text")]
    text: Option<String>,
}

fn read_text(text: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
    match text {
        Some(text) => Ok(text),
        None => {
            let mut text = String::new();
            io::stdin().read_to_string(&mut text)?;
            Ok(text)
        }
    }
}

fn rot13(text: &str) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                (((c as u8 - base + 13) % 26) + base) as char
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    let args = Args::parse();
    match read_text(args.text) {
        Ok(text) => println!("{}", rot13(&text)),
        Err(e) => eprintln!("Error: {}", e),
    }
}
