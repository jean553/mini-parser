//! Parser main file 

use std::env;
use std::process;
use std::io::stderr;
use std::io::Write;

/// Check if the given character is a digit (between 0 and 9) 
///
/// # Arguments
///
/// * `character` - character to check
fn is_digit(character: char) -> bool {
    let digits: [char; 10] = ['0','1','2','3','4','5','6','7','8','9'];
    return digits.contains(&character);
}

/// Check if the given character is a plus (+)
///
/// # Arguments
///
/// * `character` - character to check
fn is_plus(character: char) -> bool {
    return character == '+';
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

            if is_digit(character) {

                output.push_str("mov eax, ");
                output.push(character);
                output.push('\n');

                iterate(iterator, output);
            }
            else if is_plus(character) {

                output.push_str("mov ebx, eax\n");

                let next_character = iterator.next();
                match next_character {
                    Some(next_character) => {
                        output.push_str("mov eax, ");
                        output.push(next_character);
                        output.push('\n');
                    }
                    None => {}
                }

                output.push_str("add eax, ebx\n");

                iterate(iterator, output);
            }
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
