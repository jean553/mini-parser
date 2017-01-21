//! Parser main file

use std::env;
use std::process;
use std::io::stderr;
use std::io::Write;

#[derive(PartialEq)]
enum Token {
    Digit,
    Plus,
    Unknown
}

/// Lexical analyze of one character and return a token for parsing.
///
/// # Arguments:
///
/// * `character` - character to check
fn get_token_from_character(character: char) -> Token {

    match character {
        '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
            return Token::Digit;
        }
        '+' => {
            return Token::Plus;
        }
        _ => {
            return Token::Unknown;
        }
    }
}

fn main() {

    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        writeln!(&mut stderr(), "No input.").unwrap();
        process::exit(1);
    }

    let mut output = String::from(
r#"
bits 64

segment .text
    global _start

_start:
"#
    );

    let mut previous = Vec::new();

    for (index, character) in args[1].chars().enumerate() {
        let token: Token = get_token_from_character(character);

        if token == Token::Digit {
            output.push_str("mov eax, ");
            output.push(character);
            output.push('\n');

            // TODO: won't work with numbers, must be improved
            if index != 0 {
                let prev: char = previous[index - 1];
                if get_token_from_character(prev) == Token::Plus {
                    output.push_str("add eax, ebx\n");
                }
            }
        }

        if token == Token::Plus {
            output.push_str("mov ebx, eax\n");
        }

        previous.push(character);
    }

    output +=
r#"
mov eax, 1
int 0x80
"#;

    print!("{}", output);
}
