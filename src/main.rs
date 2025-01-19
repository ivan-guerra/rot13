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
