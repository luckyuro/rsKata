extern crate kata;

use kata::katas::divisors::divisors;

#[test]
fn correct_divisor() {
    assert_eq!(divisors(15), Ok(vec![3,5]));
    assert_eq!(divisors(12), Ok(vec![2,3,4,6]));
    assert_eq!(divisors(13), Err("13 is prime".to_string()));
}