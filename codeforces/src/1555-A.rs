use std::{
    fmt::Write,
    io::{stdin, stdout, BufRead, BufWriter},
    str::FromStr,
};

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}

impl Scanner {
    fn next<T: FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("failed parse");
            }

            let mut input = String::new();
            stdin().read_line(&mut input).expect("failed read");
            // There is only a single entry per line
            // self.buffer = input.split_whitespace().rev().map(String::from).collect();
            self.buffer = input.split_whitespace().collect()
        }
    }
}

fn prompt<R, W>(mut reader: R, mut writer: W, question: &str) -> String
where
    R: BufRead,
    W: Write,
{
    write!(&mut writer, "{}", question).expect("Unable to write");
    let mut s = String::new();
    reader.read_line(&mut s).expect("Unable to read");
    s
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_pizzaforces() {
        let mut scan = Scanner::default();
        let out = &mut BufWriter::new(stdout());

        let n = scan.next::<usize>();
        let q = scan.next::<usize>();
    }
}

// #[allow(unused_imports)]
// use std::cmp::{min,max};
// use std::io::{BufWriter, stdin, stdout, Write};
// const BITS: usize = 19;
//
// #[derive(Default)]
// struct Scanner {
//     buffer: Vec<String>
// }
// impl Scanner {
//     fn next<T: std::str::FromStr>(&mut self) -> T {
//         loop {
//             if let Some(token) = self.buffer.pop() {
//                 return token.parse().ok().expect("Failed parse");
//             }
//             let mut input = String::new();
//             stdin().read_line(&mut input).expect("Failed read");
//             self.buffer = input.split_whitespace().rev().map(String::from).collect();
//         }
//     }
// }
//
// fn main() {
//     let mut scan = Scanner::default();
//     let out = &mut BufWriter::new(stdout());
//
//     let n = scan.next::<usize>();
//     let q = scan.next::<usize>();
//     let a: Vec<usize> = (0..n).map(|_| scan.next()).collect();
//
//     let mut closest = vec![n; BITS];
//     let mut next = vec![vec![n; BITS]; n+1];
//
//     for (i, &ai) in a.iter().enumerate().rev() {
//         let (zero_bits, one_bits): (Vec<usize>, Vec<usize>) =
//             (0..BITS).partition(|b| (ai & (1usize << b)) == 0);
//
//         for b in one_bits {
//             next[i][b] = i;
//             let j = closest[b];
//             for &c in &zero_bits { // must borrow (not consume) zero_bits to reuse it!
//                 // inner loop's constant factor <= 1/4, so it may run 27 million times
//                 next[i][c] = min(next[i][c], next[j][c]);
//             }
//             closest[b] = i;
//         }
//     }
//
//     for _ in 0..q {
//         let x = scan.next::<usize>() - 1;
//         let y = scan.next::<usize>() - 1;
//         let success = (0..BITS).filter(|b| (a[y] & (1usize << b)) != 0)
//                                .any(|b| next[x][b] <= y);
//         if success {
//             writeln!(out, "Shi").ok();
//         } else {
//             writeln!(out, "Fou").ok();
//         }
//     }
//
