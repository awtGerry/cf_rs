pub fn main() {
    let mut _l1 = String::new();
    std::io::stdin()
        .read_line(&mut _l1)
        .unwrap();
    let mut _v: Vec<&str> = _l1.trim()
        .split("+")
        .collect();
    _v.sort();
    print!("{}", _v.join("+"));
}
