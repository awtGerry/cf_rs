pub fn main() {
    let mut _l = String::new();
    std::io::stdin()
        .read_line(&mut _l)
        .expect("err");
    let n = _l.trim().to_string()
        .chars()
        .into_iter()
        .filter(|&x| x == '4' || x == '7')
        .count();
    if n == 4 || n == 7 {
        print!("YES");
    } else {
        print!("NO");
    }
}
