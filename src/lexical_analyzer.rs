/// Check if the given character is a digit (between 0 and 9)
///
/// # Arguments
///
/// * `character` - character to check
pub fn is_digit(character: char) -> bool {
    let digits: [char; 10] = ['0','1','2','3','4','5','6','7','8','9'];
    return digits.contains(&character);
}

/// Check if the given character is a plus (+)
///
/// # Arguments
///
/// * `character` - character to check
pub fn is_plus(character: char) -> bool {
    return character == '+';
}

