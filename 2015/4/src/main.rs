
use md5;

const SECRET: &'static str = "iwrupvqb";

fn main() {
    let mut starts_with_5 = 0;
    let mut starts_with_6 = 0;
    for i in 0..i32::MAX {
        if i%1_000_000 == 0 {
            println!("Searched: {i} numbers")
        }
        let hash = md5::compute(SECRET.to_owned()+format!("{i}").as_str());
        let hashstr = format!("{:X}", hash);
        if hashstr.starts_with("00000") {
            if starts_with_5 == 0 {starts_with_5 = i};
            if hashstr.starts_with("000000") {
                starts_with_6 = i;
                break
            }
        }

    }
    println!("Hash offset that starts with 5: {starts_with_5}"); // 346386
    println!("Hash offset that starts with 6: {starts_with_6}"); // 9958218
}
