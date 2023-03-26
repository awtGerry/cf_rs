pub fn main() {
    let mut _l1 = String::new();
    std::io::stdin()
        .read_line(&mut _l1)
        .expect("err");
    print!("{}", _l1[0..1].to_uppercase() + &_l1[1..]);
}
