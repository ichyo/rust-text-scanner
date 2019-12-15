// to support 1.15.1
#![allow(unknown_lints)]
#![allow(renamed_and_removed_lints)]
#![allow(redundant_field_names)]

use std::fmt::Debug;
use std::io::{self, BufRead, Read};
use std::str::FromStr;

#[macro_export]
macro_rules! scan {
    ($($t:tt),*) => {{
        let stdin = std::io::stdin();
        read!(stdin.lock(), $($t),*)
    }};
}

#[macro_export]
macro_rules! fscan {
    ($r:expr, $($t:tt),*) => {{
        let mut sc = Scanner::new(&mut $r);
        _fscan!(sc, $($t),*)
    }}
}

#[macro_export]
macro_rules! _fscan {
    ($r:expr, [char]) => {
        _fscan!($r, String).chars().collect::<Vec<char>>()
    };
    ($r:expr, [($($t:ty),*); $n:expr]) => {
        (0..$n).map(|_| _fscan!($r, ($($t),*))).collect::<Vec<_>>()
    };
    ($r:expr, [$t:ty; $n:expr]) => {
        (0..$n).map(|_| _fscan!($r, $t)).collect::<Vec<$t>>()
    };
    ($r:expr, ($($t:ty),*)) => {
        ($(_fscan!($r, $t)),*)
    };
    ($r:expr, $t:ty) => {
        $r.scan::<$t>().expect("EOF")
    };
}

pub fn readln() -> Result<Option<String>, io::Error> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    freadln(&mut stdin)
}

pub fn freadln<R: BufRead>(r: &mut R) -> Result<Option<String>, io::Error> {
    let mut buf = String::new();
    let length = r.read_line(&mut buf)?;
    if let Some('\n') = buf.chars().last() {
        buf.pop();
    }
    if let Some('\r') = buf.chars().last() {
        buf.pop();
    }
    if length == 0 {
        Ok(None)
    } else {
        Ok(Some(buf))
    }
}

fn is_ascii_whitespace(b: u8) -> bool {
    // Can use u8::is_ascii_whitespace once removing support of 1.15.1
    match b {
        b'\t' | b'\n' | b'\x0C' | b'\r' | b' ' => true,
        _ => false,
    }
}

pub struct Tokenizer<'a, R: Read + 'a> {
    reader: &'a mut R,
}

pub struct Scanner<'a, R: Read + 'a> {
    tokenizer: Tokenizer<'a, R>,
}

impl<'a, R: Read> Tokenizer<'a, R> {
    pub fn new(reader: &'a mut R) -> Self {
        Tokenizer { reader: reader }
    }

    pub fn next_token(&mut self) -> Option<String> {
        let token: Vec<u8> = self
            .reader
            .by_ref()
            .bytes()
            .map(|r| r.expect("IO error"))
            .skip_while(|&b| is_ascii_whitespace(b))
            .take_while(|&b| !is_ascii_whitespace(b))
            .collect();
        if token.is_empty() {
            None
        } else {
            Some(String::from_utf8(token).expect("UTF-8 encoding error"))
        }
    }
}

impl<'a, R: Read> Scanner<'a, R> {
    pub fn new(reader: &'a mut R) -> Self {
        Scanner {
            tokenizer: Tokenizer::new(reader),
        }
    }

    pub fn scan<T>(&mut self) -> Option<T>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        self.tokenizer
            .next_token()
            .map(|s| s.parse::<T>().expect("parse error"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_read() {
        let mut buffer = Cursor::new(b"-10\n1.1\n");
        assert_eq!(-10i64, fscan!(buffer, i64));
        assert_eq!(1.1f64, fscan!(buffer, f64));

        let mut buffer = Cursor::new(b"-10\n1.1\n");
        assert_eq!((-10i64, 1.1f64), fscan!(buffer, (i64, f64)));

        let mut buffer = Cursor::new(b"-10\n1.1\n");
        assert_eq!(vec![(-10i64, 1.1f64)], fscan!(buffer, [(i64, f64); 1]));

        let mut buffer = Cursor::new(b"-10\n1.1\n");
        assert_eq!(vec![-10f64, 1.1f64], fscan!(buffer, [f64; 2]));

        let mut buffer = Cursor::new(b"-10\n1.1\n");
        assert_eq!(vec!['-', '1', '0'], fscan!(buffer, [char]));
    }

    #[test]
    fn test_readln() {
        let mut buffer = Cursor::new(b"-10\n1.1\r\n");
        assert_eq!(Some("-10".to_string()), freadln(&mut buffer).unwrap());
        assert_eq!(Some("1.1".to_string()), freadln(&mut buffer).unwrap());
        assert_eq!(None, freadln(&mut buffer).unwrap());
    }

    #[test]
    fn test_scanner() {
        let mut buffer: &[u8] = b"-10\n1.1\n";
        let mut sc = Scanner::new(&mut buffer);
        assert_eq!(sc.scan::<i64>(), Some(-10));
        assert_eq!(sc.scan::<f64>(), Some(1.1));
        assert_eq!(sc.scan::<f64>(), None);
    }

    #[test]
    fn test_next_token() {
        let mut buffer: &[u8] = b"ab \nc d \n";
        let mut tk = Tokenizer::new(&mut buffer);
        assert_eq!(tk.next_token(), Some("ab".to_string()));
        assert_eq!(tk.next_token(), Some("c".to_string()));
        assert_eq!(tk.next_token(), Some("d".to_string()));
        assert_eq!(tk.next_token(), None);
    }

    #[test]
    fn test_next_token_empty_lines() {
        let mut buffer: &[u8] = b"\n\n";
        let mut tk = Tokenizer::new(&mut buffer);
        assert_eq!(tk.next_token(), None);
    }
}
