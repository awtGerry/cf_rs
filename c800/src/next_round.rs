pub fn main() {
    let mut _l1 = String::new();
    std::io::stdin()
        .read_line(&mut _l1)
        .unwrap();

    let _v1: Vec<&str> = _l1.trim()
        .split_whitespace()
        .collect();
    let contestants: usize = _v1[0].parse().unwrap();
    let kth: usize = _v1[1].parse().unwrap();

    let mut nums = String::new();
    std::io::stdin()
        .read_line(&mut nums)
        .unwrap();
    let _v2: Vec<u16> = nums.trim()
        .split(' ')
        .map(|e| e.parse().unwrap())
        .collect();

    let mut res = 0;
    for i in 0..contestants {
        if _v2[i] >= _v2[kth - 1] {
            res += 1;
        }
    }
    print!("{res}");
}
