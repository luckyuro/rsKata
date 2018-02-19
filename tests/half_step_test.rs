extern crate kata;

use kata::katas::half_step::solution;

#[test]
fn half_step_tests() {
    assert_eq!(solution(4.2), 4.0);
    assert_eq!(solution(4.4), 4.5);
    assert_eq!(solution(4.6), 4.5);
    assert_eq!(solution(4.8), 5.0);
}