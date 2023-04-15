fn solve<R: BufRead, W: Write>(line: &mut Scanner<R>, w: &mut W) {
    let n = line.token::<u32>();
    let h = line.token::<u32>();
    let mut width: u32 = 0;
    for _ in 0..n {
        let v = line.token::<u32>();
        if (v <= h) {
            width+=1;
        } else {
            width+=2;
        }
    }
    writeln!(w, "{width}");
}
