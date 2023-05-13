fn solve<R: BufRead, W: Write>(line: &mut Scanner<R>, w: &mut W) {
    let n = line.token::<usize>();
    let h = line.token::<usize>();
    let mut ans: i32 = 0;
    for _ in (0..n) {
        let current = line.token::<usize>();
        if (current > h) {
            ans += 2;
        } else {
            ans += 1;
        }
    }
    writeln!(w, "{ans}");
}
