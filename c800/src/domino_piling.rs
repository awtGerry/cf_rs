pub fn main() {
    let mut lines = String::new();
    std::io::stdin()
        .read_line(&mut lines)
        .unwrap();
    let _v: Vec<u16> = lines.trim()
        .split_whitespace()
        .map(|e| e.parse().unwrap())
        .collect();
    let m = _v[0];
    let n = _v[1];
    let res = m * n / 2;
    println!("{res}");
}
