//6kyu
//kata_URL:
//         https://www.codewars.com/kata/replace-with-alphabet-position/train/rust

pub fn alphabet_position(text: &str) -> String {
    // Code here...
//    let mut ans = vec![];
//    for x in text.to_ascii_lowercase().as_bytes().iter(){
//        println!("{}", x);
//        if *x > 96  && *x < 123 {
//            ans.push(format!("{}", (*x) -97 + 1))
//        }
//    };
//    ans.join(" ")

    text.to_lowercase()
        .chars()
        .filter(|c| c>=&'a' && c<=&'z')
        .map(|c| (c as u32 - 96).to_string())
        .collect::<Vec<String>>()
        .join(" ")
}
