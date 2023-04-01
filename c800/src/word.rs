pub fn main() {
    let mut _l = String::new();
    std::io::stdin()
        .read_line(&mut _l)
        .expect("err");
    _l = _l.trim().to_string();
    let mut words = 0;
    for i in _l.chars() {
        if i.is_uppercase() {
            words +=1;
        }
    }

    if _l.len() / 2 >= words {
        print!("{}", _l.to_lowercase());
    } else {
        print!("{}", _l.to_uppercase());
    }

}
