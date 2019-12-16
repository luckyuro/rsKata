//6kyu
//kata_URL:
//          https://www.codewars.com/kata/5592e3bd57b64d00f3000047/train/rust

#[allow(dead_code)]
pub fn find_nb(n: u64) -> i32 {
    let inner = n*4;
    let bot = (inner as f64).sqrt().sqrt() as u64;

    if (bot*(bot+1)) as f64 == (inner as f64).sqrt(){
        bot as i32
    } else {
        -1
    }
}

//fn find_nb(n: u64) -> i32 {
//    let kk = (4.0 * n as f64).sqrt().sqrt().floor() as u64;
//    if 4u64 * n == kk * kk * (kk + 1u64) * (kk + 1u64)
//        {kk as i32}
//        else
//        {-1}
//}