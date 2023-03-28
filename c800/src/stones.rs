pub fn main() {
    let mut _l = String::new();
    std::io::stdin()
        .read_line(&mut _l)
        .expect("err");
    let num = _l.trim().parse::<usize>().unwrap();

    _l.clear();
    std::io::stdin()
        .read_line(&mut _l)
        .expect("err");

    let mut c: u8 = 0;
    for i in 0..num {
        if _l.chars().nth(i).unwrap() == _l.chars().nth(i+1).unwrap() {
            c+=1;
        }
    }
    print!("{c}");
}
