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

fn main() {
    
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        writeln!(&mut stderr(), "No input.").unwrap();
        process::exit(1);
    }

    for character in args[1].chars() {

        if is_digit(character)
        {
        }
    }
}
