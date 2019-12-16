//5kyu
//kata_URL:
//          https://www.codewars.com/kata/is-my-friend-cheating/train/rust

pub  fn remove_nb(m: i32) -> Vec<(i32, i32)> {
    // your code
    let m = m as usize;
    let sum = (1 + m ) * m / 2;
    let mut ans = vec![];

    for x in sum/m..m {
        let y = (sum - x ) / (1 + x);
        if x * y == sum - x - y {
            ans.push((x as i32, y as i32))
        }
    }

//    {
//        let mut ret = vec![];
//
//        for (x, y) in ans.iter() {
//            ret.push((*x, *y))
//        }
//
//        for (x, y) in ans.iter().rev() {
//            ret.push((*y, *x))
//        }
//
//        ret
//    }
    ans
}