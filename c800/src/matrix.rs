pub fn beauty() {
    let mut _l1 = String::new();
    let mut n1: i8 = 0;
    let mut n2: i8 = 0;
    for i in 0..5 {
        std::io::stdin()
            .read_line(&mut _l1)
            .unwrap();
        let _vec: Vec<i8> = _l1.trim()
            .split(' ')
            .map(|e| e.parse().unwrap())
            .collect();
        for j in 0..5 {
            if *_vec.get(j).unwrap() == 1 {
                n1 = ((i as i8) -2).abs();
                n2 = ((j as i8) -2).abs();
            }
        }
        _l1.clear();
    }
    println!("{}", n1 + n2);
}
