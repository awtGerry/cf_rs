pub fn main() {
    let mut _l = String::new();
    std::io::stdin()
        .read_line(&mut _l)
        .expect("err");
    let _v: Vec<u16> = _l.trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();
}
