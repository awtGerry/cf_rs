use std::io;

fn watermelon() {
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

fn way_too_long_words(s: String) {
    let n = s.len()-3;
    if n > 10 {
        let s1 = s.chars().nth(0).unwrap();
        let s2 = s.chars().nth(n-1).unwrap();
        println!("{}{}{}",s1,n,s2);
    } else {
        println!("{}", s);
    }
}

fn main() {
    let mut word = String::new();
    io::stdin().read_line(&mut word).expect("err");
    way_too_long_words(word);
}
