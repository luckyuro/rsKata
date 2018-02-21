//6kyu
//kata_URL:
//          https://www.codewars.com/kata/consecutive-strings/train/rust

#[allow(dead_code)]
pub fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
    let l = strarr.len();
    if l == 0 || k > l || k <= 0 {
        return "".to_string();
    }

    let mut max = 0;
    for i in 0..k {
        max += strarr[i].len();
    }

    let mut sum = max;
    let mut max_i = 0;

    for i in 1..l-k+1 {
        sum = sum - strarr[i-1].len() + strarr[i+k-1].len();
        if sum > max {
            max = sum;
            max_i = i;
        }
    }

    let mut ret = "".to_string();
    for i in 0..k{
        ret += &strarr[max_i+i].to_string();
    }

    ret

}


//fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
//    let mut result = String::new();
//
//    if k > 0 && strarr.len() >= k {
//        for index in 0..strarr.len() - k + 1 {
//            let s: String = strarr[index..index + k].join("");
//            if s.len() > result.len() {
//                result = s;
//            }
//        }
//    }
//
//    result
//}