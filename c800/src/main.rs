#![allow(unused)]
#![allow(unused_imports)]

use std::io::{self, prelude::*};
use std::cmp::Ordering::{Less, Equal, Greater};
use std::collections::{HashSet, HashMap};
use std::cmp::{min,max,Reverse};
use std::str;

/**** TEMPLATE ****/
struct Scanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitWhitespace<'static>,
}

impl<R: BufRead> Scanner<R> {
    fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_whitespace()
        }
    }

    fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Error parsing");
            }
            self.buf_str.clear();
            self.reader.read_until(b'\n', &mut self.buf_str).expect("Error reading");
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_whitespace())
            }
        }
    }
}
/**** END OF TEMPLATE ****/

fn solve<R: BufRead, W: Write>(line: &mut Scanner<R>, w: &mut W) {
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    solve(&mut scan, &mut out);
}
