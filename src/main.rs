//! Parser main file

use std::env;
use std::process;
use std::io::stderr;
use std::io::Write;

mod lexical_analyzer;

/// Generates assembly instructions when a digit is met
///
/// # Arguments:
///
/// * `ouput` - String object to generate the ouput assembly
/// (mutable reference in order to add code)
/// * `character` - character to check
fn read_digit(
    output: &mut std::string::String,
    character: char
) {
    output.push_str("mov eax, ");
    output.push(character);
    output.push('\n');
}

/// Generates assembly instructions when a '+' is met
///
/// # Arguments:
///
/// * `iterator` - iterator over the input characters
/// (mutable reference in order to iterate)
/// * `ouput` - String object to generate the ouput assembly
/// (mutable reference in order to add code)
fn read_plus(
    iterator: &mut std::str::Chars,
    output: &mut std::string::String
) {
    output.push_str("mov ebx, eax\n");

    let character = iterator.next();
    match character {
        Some(character) => {
            read_digit(output, character);
        }
        None => {}
    }

    output.push_str("add eax, ebx\n");
}

/// Check if the given character matches with a symbol type
///
/// # Arguments:
///
/// * `iterator` - iterator over the input characters
/// (mutable reference in order to iterate)
/// * `ouput` - String object to generate the ouput assembly
/// (mutable reference in order to add code)
/// * `character` - character to check
fn handle_character(
    iterator: &mut std::str::Chars,
    output: &mut std::string::String,
    character: char
)
{
    let character_is_digit: bool = lexical_analyzer::is_digit(character);
    let character_is_plus: bool = lexical_analyzer::is_plus(character);

    if !character_is_digit && !character_is_plus {
        return;
    }

    if character_is_digit {
        read_digit(output, character);
    }

    if character_is_plus {
        read_plus(iterator, output);
    }

    iterate(iterator, output);
}

/// Recursive method that reads the given instructions from 'iterator'
/// and fills the 'output' string object with assembly code
///
/// # Arguments
///
/// * `iterator` - iterator over the input characters
/// (mutable reference in order to iterate)
/// * `ouput` - String object to generate the ouput assembly
/// (mutable reference in order to add code)
fn iterate(
    iterator: &mut std::str::Chars,
    output: &mut std::string::String
) {
    let character = iterator.next();
    match character {
        Some(character) => {
            handle_character(iterator, output, character);
        }
        None => {}
    };
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

    let mut iterator = args[1].chars();
    iterate(&mut iterator, &mut output);

    output +=
r#"
mov eax, 1
int 0x80
"#;

    print!("{}", output);
}
