//! A [ROT13](https://en.wikipedia.org/wiki/ROT13) cipher implementation in Rust.
//!
//! ROT13 is a simple letter substitution cipher that replaces a letter with
//! the 13th letter after it in the alphabet. ROT13 is a special case of the
//! [Caesar Cipher](https://en.wikipedia.org/wiki/Caesar_cipher).
//!
//! # Features
//!
//! - Supports both uppercase and lowercase letters
//! - Preserves non-alphabetic characters
//! - Accepts input from both command line arguments and stdin
//! - Implements symmetric encryption (applying ROT13 twice returns the original text)
//!
//! # Example
//!
//! Basic usage:
//! ```
//! # use rot13::rot13;
//! let original = "Hello, World!";
//! assert_eq!(rot13(original), "Uryyb, Jbeyq!");
//! ```
//!
//! ROT13 is its own inverse - applying it twice returns the original text:
//! ```
//! # use rot13::rot13;
//! let text = "The quick brown fox";
//! let encoded = rot13(text);
//! let decoded = rot13(&encoded);
//! assert_eq!(decoded, text);
//! ```
//!
//! Non-alphabetic characters remain unchanged:
//! ```
//! # use rot13::rot13;
//! assert_eq!(rot13("123!@#"), "123!@#");
//! assert_eq!(rot13("Hello, 2023!"), "Uryyb, 2023!");
//! ```
//!
//! Case is preserved:
//! ```
//! # use rot13::rot13;
//! assert_eq!(rot13("aBcDeF"), "nOpQrS");
//! ```
//!
//! # Command Line Usage
//!
//! ```text
//! $ echo "Hello, World!" | rot13
//! Uryyb, Jbeyq!
//! ```
//!
//! Or directly as an argument:
//!
//! ```text
//! $ rot13 "Hello, World!"
//! Uryyb, Jbeyq!
//! ```
use clap::Parser;
use std::io::{self, Read};

#[doc(hidden)]
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(help = "ROT13 text")]
    text: Option<String>,
}

#[doc(hidden)]
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

#[doc(hidden)]
fn rot13(text: &str) -> String {
    const ROT13_SHIFT: u8 = 13;
    const ALPHABET_SIZE: u8 = 26;
    const LOWERCASE_A: u8 = b'a';
    const UPPERCASE_A: u8 = b'A';

    text.chars()
        .map(|c| match c {
            'a'..='z' => {
                (((c as u8 - LOWERCASE_A + ROT13_SHIFT) % ALPHABET_SIZE) + LOWERCASE_A) as char
            }
            'A'..='Z' => {
                (((c as u8 - UPPERCASE_A + ROT13_SHIFT) % ALPHABET_SIZE) + UPPERCASE_A) as char
            }
            _ => c,
        })
        .collect()
}

#[doc(hidden)]
fn main() {
    let args = Args::parse();
    match read_text(args.text) {
        Ok(text) => println!("{}", rot13(&text)),
        Err(e) => eprintln!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rot13_lowercase() {
        assert_eq!(rot13("hello"), "uryyb");
    }

    #[test]
    fn test_rot13_uppercase() {
        assert_eq!(rot13("HELLO"), "URYYB");
    }

    #[test]
    fn test_rot13_mixed_case() {
        assert_eq!(rot13("Hello World"), "Uryyb Jbeyq");
    }

    #[test]
    fn test_rot13_numbers_unchanged() {
        assert_eq!(rot13("123"), "123");
    }

    #[test]
    fn test_rot13_punctuation_unchanged() {
        assert_eq!(rot13("Hello, World!"), "Uryyb, Jbeyq!");
    }

    #[test]
    fn test_rot13_empty_string() {
        assert_eq!(rot13(""), "");
    }

    #[test]
    fn test_rot13_double_rotation() {
        let text = "The quick brown fox";
        assert_eq!(rot13(rot13(text).as_str()), text);
    }

    #[test]
    fn test_rot13_special_characters() {
        assert_eq!(rot13("@#$%^&*"), "@#$%^&*");
    }
}
