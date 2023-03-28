pub fn main() {
    let mut _l = String::new();
    std::io::stdin()
        .read_line(&mut _l)
        .expect("err");
    let nums: Vec<u16> = _l.trim()
        .split(" ")
        .map(|x| x.parse().unwrap())
        .collect();
    let k = nums[0];
    let mut sum: u16 = 0;
    for i in 1..nums[2]+1 {
        sum += i*k;
    }
    print!("{}", sum-nums[1]);
}
