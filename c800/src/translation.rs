fn solve<R: BufRead, W: Write>(line: &mut Scanner<R>, w: &mut W) {
    let _l1 = line.token::<String>();
    let _l2 = line.token::<String>();
    let reverse: String = t.chars().rev().collect();
    if (_l1 == reverse) {
        writeln!(w, "YES");
    } else {
        writeln!(w, "NO");
    }
}
