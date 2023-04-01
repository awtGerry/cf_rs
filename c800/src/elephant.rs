pub fn main() {
    let mut _l = String::new();
    std::io::stdin()
        .read_line(&mut _l)
        .unwrap();
    let x: i32 = _l.trim().parse().unwrap();
    let n: i32;
    if x % 5 == 0 {
        n = x/5;
    } else {
        n = (x/5)+1;
    }
    print!("{n}");
}
