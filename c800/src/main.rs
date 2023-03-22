use std::io;

mod too_long_words;

fn main() {
    let mut word = String::new();
    io::stdin().read_line(&mut word).expect("err");
    too_long_words::way_too_long_words(word);
}
