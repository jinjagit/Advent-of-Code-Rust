// It is a six-digit number.
// The value is within the range given in your puzzle input. (284639-748759)
// Two adjacent digits are the same (like 22 in 122345).
// Going from left to right, the digits never decrease; they only ever increase or stay the same
// (like 111123 or 135679).
// AND the two adjacent matching digits are not part of a larger group of matching digits.

fn main() {
    let mut value: u32 = 284639;
    let mut candidate_counter: u32 = 0;

    // loop (until upper range limit reached)
    //   start at left digit and iterate over digits to penultimate digit:
    //     if next digit is lower, then next digit = current digit
    //   iterate (again) over resultant digits:
    //     if find isolated pair of matching digits, increment counter of candidate passwords
    //   add 1 to value

    while value < 748760 {
        let mut digits: Vec<u8> = number_to_digits(value);

        for i in 0..5 {
            if digits[i + 1] < digits[i] {
                digits[i + 1] = digits[i];
            }
        }

        if contains_isolated_pair(&digits) == true {
            candidate_counter += 1;
        }

        value = digits_to_number(digits) + 1;
    }

    println!("candidate passwords: {}", candidate_counter);
}

// converts a vec of 6 single-digit integers to the integer represented by the digit sequence
fn digits_to_number(digits: Vec<u8>) -> u32 {
    let mut value: u32 = 0;

    for i in 0..6 {
        value = value + (digits[5 - i] as u32 * 10u32.pow(i as u32));
    }

    value
}

// converts a 6-digit integer to a vec of sequential integer single-digits
fn number_to_digits(num: u32) -> Vec<u8> {
    let mut digits: Vec<u8> = vec![0; 6];
    let mut divisor: f32 = 100000.0;
    let mut remainder = num;

    for i in 0..6 {
        digits[i] = (remainder as f32 / divisor).floor() as u8;
        remainder -= digits[i] as u32 * divisor as u32;
        divisor /= 10.0;
    }

    digits
}

fn contains_isolated_pair(digits: &Vec<u8>) -> bool {
    let mut isolated_pair: bool = false;

    for i in 0..5 {
        if &digits[i] == &digits[i + 1]
            && (i == 0 || &digits[i] != &digits[i - 1])
            && (i >= 4 || &digits[i] != &digits[i + 2])
        {
            isolated_pair = true;
        }
    }

    isolated_pair
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn contains_isolated_pair_test() {
        let digits = vec![1, 1, 2, 2, 3, 3];
        let digit_pair = contains_isolated_pair(&digits);
        assert_eq!(digit_pair, true);

        let digits = vec![1, 2, 3, 4, 4, 4];
        let digit_pair = contains_isolated_pair(&digits);
        assert_eq!(digit_pair, false);

        let digits = vec![1, 1, 1, 1, 2, 2];
        let digit_pair = contains_isolated_pair(&digits);
        assert_eq!(digit_pair, true);
    }
}
