//6kyu
//kata_URL:
//          https://www.codewars.com/kata/are-they-the-same/train/rust

#[allow(dead_code)]
pub fn comp(a: Vec<i64>, b: Vec<i64>) -> bool {
    let mut new_a:Vec<i64> = a
        .into_iter()
        .map(|x| x*x)
        .collect();
    new_a.sort();

    let mut new_b = b;
    new_b.sort();

    new_a == new_b
}

//fn comp(a: Vec<i64>, b: Vec<i64>) -> bool {
//    let mut a1 = a.iter().map(|&x| x * x).collect::<Vec<_>>();
//    let mut a2 = b;
//    a1.sort();
//    a2.sort();
//    a1 == a2
//}

//fn comp(mut a: Vec<i64>, mut b: Vec<i64>) -> bool {
//    for x in a.iter_mut() {
//        *x *= *x;
//    }
//    a.sort();
//    b.sort();
//    a == b
//}