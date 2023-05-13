#![allow(unused)]
#![allow(unused_imports)]

use std::io::{self, prelude::*};
use std::cmp::Ordering::{Less, Equal, Greater};
use std::collections::{HashSet, HashMap};
use std::cmp::{min,max,Reverse};
use std::str;

/**** TEMPLATE ****/
/** from: https://codeforces.com/profile/EbTech **/
struct Scanner<R> {
    reader: R,
    line: Vec<u8>,
    iter: str::SplitWhitespace<'static>,
}

impl<R: BufRead> Scanner<R> {
    fn new(reader: R) -> Self {
        Self {
            reader,
            line: vec![],
            iter: "".split_whitespace()
        }
    }

    fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.iter.next() {
                return token.parse().ok().expect("Error parsing");
            }
            self.line.clear();
            self.reader.read_until(b'\n', &mut self.line).expect("Error reading");
            self.iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.line);
                std::mem::transmute(slice.split_whitespace())
            }
        }
    }
}
/**** END OF TEMPLATE ****/

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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scanner = Scanner::new(stdin.lock());
    let mut writer = io::BufWriter::new(stdout.lock());
    solve(&mut scanner, &mut writer);
}
