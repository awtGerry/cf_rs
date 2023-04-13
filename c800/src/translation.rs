fn solve<R: BufRead, W: Write>(line: &mut Scanner<R>, w: &mut W) {
    let s = line.token::<String>();
    let t = line.token::<String>();
    let rt: String = t.chars().rev().collect();
    if (s == rt) {
        writeln!(w, "YES");
    } else {
        writeln!(w, "NO");
    }
}
