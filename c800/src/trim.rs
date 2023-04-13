pub fn main() {
    let mut inp = String::new();
    std::io::stdin()
        .read_line(&mut inp)
        .unwrap();
    let num = inp.trim().parse::<usize>().unwrap();
    let mut c: u16 = 0;
    let mut max: u16 = 0;
    for _ in 0..num {
        inp.clear();
        std::io::stdin()
            .read_line(&mut inp)
            .unwrap();
            let _v: Vec<u16> = inp.trim().split(' ').map(|x| x.parse::<u16>().unwrap()).collect();
            c += _v[1];
            c -= _v[0];
            if c > max {
                max = c;
            }
    }
    println!("{max}");
}
