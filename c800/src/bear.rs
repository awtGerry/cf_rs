pub fn main() {
    let mut _l = String::new();
    std::io::stdin()
        .read_line(&mut _l)
        .unwrap();
    let _v: Vec<usize> = _l.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut a = _v[0];
    let b = _v[1];
    let mut years: u8 = 1;
    while a < b {
        a *= 2;
        years+=1;
    }
    println!("{years}");
}
