//6kyu
//kata_URL:
//          https://www.codewars.com/kata/playing-on-a-chessboard/train/rust

#[allow(dead_code)]
pub fn game(n: u64) -> Vec<u64> {
    let odd = n % 2;
    let ans = (1+odd + n-1)*(n/2);
    if odd == 1 {
        vec![ans*2+1, 2]
    } else {
        vec![ans]
    }
}


//fn game(n: u64) -> Vec<u64> {
//    if n % 2 == 0 {
//        return vec![n/2*n];
//    }
//
//    return vec![n*n, 2];
//}

//fn game(n: u64) -> Vec<u64> {
//    let n = n as f64;
//    let m: f64 = (n*n)/2.0;
//    let (num, frac) = (m.trunc(), m.fract());
//    let (num, den) = (num as u64, (1.0 / frac) as u64);
//    if den == 0 {
//        vec![num]
//    } else {
//        vec![num * den + 1, den]
//    }
//}