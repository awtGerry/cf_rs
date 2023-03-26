pub fn main() {
    let mut _l1 = String::new();
    std::io::stdin()
        .read_line(&mut _l1)
        .unwrap();
    let mut _l2 = String::new();
    std::io::stdin()
        .read_line(&mut _l2)
        .unwrap();
    _l1 = _l1.to_lowercase();
    _l2 = _l2.to_lowercase();
    if _l1 == _l2 {
        println!("0");
    } else if _l1 > _l2 {
        println!("1");
    } else {
        println!("-1");
    }
}
