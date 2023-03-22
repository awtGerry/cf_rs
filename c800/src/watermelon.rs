pub fn wm() {
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("err");
    let w: u8 = inp.trim().parse().expect("not a number");
    if w % 2 == 0 && w != 0 {
        println!("YES");
    } else {
        println!("NO");
    }
}
