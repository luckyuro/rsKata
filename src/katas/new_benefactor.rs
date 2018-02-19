//7kyu
//kata_URL:
//          https://www.codewars.com/kata/569b5cec755dd3534d00000f/train/rust

#[allow(dead_code)]
pub fn new_avg(arr: &[f64], newavg: f64) -> Option<i32> {
    let mut ans:f64 = newavg;

    for i in arr.iter(){
        ans += newavg - i;
    }

    if ans <= 0.0 {
        return None;
    } else {
        return Some(ans.ceil() as i32);
    }
}

//fn new_avg(arr: &[f64], newavg: f64) -> Option<i32> {
//    match (arr.len() as f64 + 1f64) * newavg - arr.iter().sum::<f64>() {
//        n if n > 0f64 => Some(n.ceil() as i32),
//        _ => None
//    }
//}