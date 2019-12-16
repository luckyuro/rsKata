//5kyu
//kata_URL:
//          http://www.codewars.com/kata/55c6126177c9441a570000cc/train/rust

//use std::cmp::Ordering;

#[allow(dead_code)]
pub fn order_weight(s: &str) -> String {
    let mut arr = s
        .split_whitespace()
        .collect::<Vec<&str>>();

    arr.sort_by(|sf, sl| {
        let former = sf.chars()
            .map(|x| x.to_digit(10).unwrap())
            .sum::<u32>();
        let later = sl.chars()
            .map(|x| x.to_digit(10).unwrap())
            .sum::<u32>();

        if former == later{
            sf.cmp(sl)
        } else {
            former.cmp(&later)
        }});
    arr.join(" ")
        .to_string()
}

//pub fn order_weight(s: &str) -> String {
//    let mut arr = s
//        .split_whitespace()
//        .collect::<Vec<&str>>();
//
//    arr.sort_by(|a,b| new_sort(a,b));
//    arr.join(" ")
//        .to_string()
//}
//
//fn new_sort(sf: &str, sl: &str) -> Ordering{
//    let former = sf.chars()
//        .map(|x| x.to_digit(10).unwrap())
//        .sum::<u32>();
//    let later = sl.chars()
//        .map(|x| x.to_digit(10).unwrap())
//        .sum::<u32>();
//
//    if former == later{
//        sf.cmp(sl)
//    } else {
//        former.cmp(&later)
//    }
//}