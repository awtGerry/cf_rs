pub fn not_pp() {
    let mut _l1 = String::new();
    std::io::stdin()
        .read_line(&mut _l1)
        .unwrap();
    let n: i8 = _l1.trim().parse().unwrap();
    let mut x: i32 = 0;
    for _ in 0..n {
        _l1.clear();
        std::io::stdin()
            .read_line(&mut _l1)
            .unwrap();
        if _l1.contains("+") {
            x += 1;
        } else {
            x -= 1;
        }
    }
    println!("{}", x);
}
