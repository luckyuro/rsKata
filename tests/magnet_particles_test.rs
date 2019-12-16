extern crate kata;
use kata::katas::magnet_particles::doubles;

fn assert_fuzzy_equals(actual: f64, expected: f64) {
    let merr = 1.0e-10;
    let inrange =
        if expected == 0.0 {
            (actual.abs() <= merr)
        } else {
            ((actual - expected).abs() / expected <= merr)
        };
    if inrange == false {
        println!("Expected value must be near: {:e} but was:{:e}", expected, actual);
    }
    assert_eq!(true, inrange);
}

fn dotest(maxk: i32, maxn: i32, exp: f64) -> () {
    assert_fuzzy_equals(doubles(maxk, maxn), exp);
}

#[test]
fn basic_tests_doubles() {
    dotest(1, 10, 0.5580321939764581);
    dotest(10, 1000, 0.6921486500921933);
    dotest(10, 10000, 0.6930471674194457);
}