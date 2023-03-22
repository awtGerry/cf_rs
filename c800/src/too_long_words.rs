pub fn way_too_long_words(s: String) {
    let n = s.len()-3;
    if n > 10 {
        let s1 = s.chars().nth(0).unwrap();
        let s2 = s.chars().nth(n-1).unwrap();
        println!("{}{}{}",s1,n,s2);
    } else {
        println!("{}", s);
    }
}

