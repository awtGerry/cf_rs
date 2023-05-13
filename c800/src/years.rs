fn solve<R: BufRead, W: Write>(line: &mut Scanner<R>, w: &mut W) {
    let y = line.token::<String>();
    let mut diff = true;
    let mut res = String::new();
    let mut cont = 1;
    while diff {
        diff = false;
        let mut _v: Vec<char> = Vec::new();
        let nums = (y.trim().parse::<u32>().unwrap() + cont).to_string();
        for i in nums.chars() {
            if _v.contains(&i) {
                diff = true;
            } else {
                _v.push(i)
            }
        }
        cont+=1;
        _v.clear();
        if !diff {
            res = nums.clone();
        }
    }
    writeln!(w, "{}", res);
}
