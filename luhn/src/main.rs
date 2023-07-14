// The Luhn algorithm is used to validate credit card numbers. The algorithm takes a string as input and does the following to validate the credit card number:

// Ignore all spaces. Reject number with less than two digits.

// Moving from right to left, double every second digit: for the number 1234, we double 3 and 1.

// After doubling a digit, sum the digits. So doubling 7 becomes 14 which becomes 5.

// Sum all the undoubled and doubled digits.

// The credit card number is valid if the sum ends with 0.

#![allow(unused_variables, dead_code)]

pub fn luhn(cc_number: &str) -> bool {
    let mut txt = String::from(cc_number);

    txt.retain(|c| c != ' ');

    if txt.len() < 2 {
        return false;
    };

    let txt: Vec<char> = txt.chars().rev().collect();

    let mut sum = 0;

    for (i, n) in txt.iter().enumerate() {
        match n.to_digit(10) {
            Some(d) => {
                sum += if i % 2 == 1 {
                    let dd = d * 2;
                    dd / 10 + dd % 10
                } else {
                    d
                }
            }
            None => {
                return false;
            }
        }
    }

    sum % 10 == 0
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {}
