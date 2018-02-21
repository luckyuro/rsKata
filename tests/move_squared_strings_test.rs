extern crate kata;

use kata::katas::move_squared_strings::hor_mirror;
use kata::katas::move_squared_strings::oper;
use kata::katas::move_squared_strings::vert_mirror;

fn testing1(s: &str, exp: &str) -> () {
    assert_eq!(oper(hor_mirror, s.to_string()), exp)
}
fn testing2(s: &str, exp: &str) -> () {
    assert_eq!(oper(vert_mirror, s.to_string()), exp)
}

#[test]
fn basics_oper() {

    testing1("lVHt\nJVhv\nCSbg\nyeCt", "yeCt\nCSbg\nJVhv\nlVHt");
    testing1("njMK\ndbrZ\nLPKo\ncEYz", "cEYz\nLPKo\ndbrZ\nnjMK");
    testing1("QMxo\ntmFe\nWLUG\nowoq", "owoq\nWLUG\ntmFe\nQMxo");

    testing2("hSgdHQ\nHnDMao\nClNNxX\niRvxxH\nbqTVvA\nwvSyRu", "QHdgSh\noaMDnH\nXxNNlC\nHxxvRi\nAvVTqb\nuRySvw");
    testing2("IzOTWE\nkkbeCM\nWuzZxM\nvDddJw\njiJyHF\nPVHfSx", "EWTOzI\nMCebkk\nMxZzuW\nwJddDv\nFHyJij\nxSfHVP");
    testing2("cuQW\nxOuD\nfZwp\neqFx", "WQuc\nDuOx\npwZf\nxFqe");

}