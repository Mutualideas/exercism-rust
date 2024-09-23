/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    //todo!("Is the Luhn checksum for {code} valid?");

    // Filter out all non-digit characters
    let filtered_string: String = code.chars().rev().filter(|c| !c.is_whitespace()).collect();

    // Return false if the input has 1 or fewer characters, or if it contains any non-digit characters
    if filtered_string.len() <= 1 || filtered_string.chars().any(|c| !c.is_ascii_digit()) {
        return false;
    }

    //double every second digit, subtract 9 if its greater than 9
    let mut sum = 0;
    let mut double_digit = false;

    // Process digits from right to left
    for digit_char in filtered_string.chars() {
        let mut digit = digit_char.to_digit(10).unwrap();

        // Double every second digit starting from the right
        if double_digit {
            digit *= 2;
            if digit > 9 {
                digit -= 9;
            }
        }
        sum += digit;
        double_digit = !double_digit; // Toggle the flag for every second digit
    }

    // check sum of digits
    sum % 10 == 0
}
