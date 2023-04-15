fn solve<R: BufRead, W: Write>(line: &mut Scanner<R>, w: &mut W) {
    let n = line.token::<usize>();
    let t = line.token::<usize>();
    let mut s: Vec<char> = line.token::<String>().chars().collect();
    for _ in 0..t {
        let mut i = 0;
        while (i+1 < n) {
            if (s[i] == 'B' && s[i+1] == 'G') {
                s.swap(i, i+1);
                i+=1;
            }
            i+=1;
        }
    }
    writeln!(w, "{}", String::from_iter(s));
}
