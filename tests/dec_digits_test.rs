extern crate kata;

use kata::katas::dec_digits::digits;

#[test]
fn digits_test() {
    assert_eq!(digits(5), 1);
    assert_eq!(digits(12345), 5);
    assert_eq!(digits(9876543210), 10);
}