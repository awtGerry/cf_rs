pub fn main() {
    let mut _l1 = String::new();
    std::io::stdin()
        .read_line(&mut _l1)
        .expect("err");
    let mut _l2 = String::new();
    std::io::stdin()
        .read_line(&mut _l2)
        .expect("err");
    let mut anton = 0;
    let mut danik = 0;
    for i in _l2.chars() {
        if i == 'A' {
            anton+=1;
        } else if i == 'D' {
            danik+=1;
        }
    }
    let output = match anton.cmp(&danik) {
        std::cmp::Ordering::Less => "Danik",
        std::cmp::Ordering::Equal => "Friendship",
        std::cmp::Ordering::Greater => "Anton",
    };
    print!("{}", output);
}
