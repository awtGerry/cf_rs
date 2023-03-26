pub fn main() {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s)
        .expect("err");
    let mut _v1: Vec<_> = s.trim()
        .chars()
        .collect();
    _v1.sort();
    _v1.dedup(); // remove duplicates
    if _v1.len() % 2 == 0 {
        print!("CHAT WITH HER");
    } else {
        print!("IGNORE HIM");
    }
}
