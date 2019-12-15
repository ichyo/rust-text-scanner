// to support 1.15.1
#![allow(unknown_lints)]
#![allow(renamed_and_removed_lints)]
#![allow(redundant_field_names)]

use std::fmt::Debug;
use std::io::Read;
use std::str::FromStr;

#[macro_export]
macro_rules! scan {
    ($($t:tt),*) => {{
        let stdin = std::io::stdin();
        read!(stdin.lock(), $($t),*)
    }};
}

#[macro_export]
macro_rules! read {
    ($r:expr, $($t:tt),*) => {{
        let mut sc = Scanner::new(&mut $r);
        _read!(sc, $($t),*)
    }}
}

#[macro_export]
macro_rules! _read {
    ($r:expr, [char]) => {
        _read!($r, String).chars().collect::<Vec<char>>()
    };
    ($r:expr, [u8]) => {
        Vec::from(read!($r, String).into_bytes())
    };
    ($r:expr, [($($t:ty),*); $n:expr]) => {
        (0..$n).map(|_| _read!($r, $($t),*)).collect::<Vec<$t>>()
    };
    ($r:expr, [$t:ty; $n:expr]) => {
        (0..$n).map(|_| _read!($r, $t)).collect::<Vec<$t>>()
    };
    ($r:expr, ($($t:ty),*)) => {
        ($(_read!($r, $t)),*)
    };
    ($r:expr, $t:ty) => {
        $r.scan::<$t>().expect("EOF")
    };
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

    pub fn next_line(&mut self) -> Option<String> {
        let mut line = Vec::new();
        for b in self.reader.by_ref().bytes().map(|r| r.expect("IO error")) {
            line.push(b);
            if b == b'\n' {
                break;
            }
        }
        if line.is_empty() {
            return None;
        }
        if let Some(&b'\n') = line.last() {
            line.pop();
        }
        if let Some(&b'\r') = line.last() {
            line.pop();
        }
        Some(String::from_utf8(line).expect("UTF-8 encoding error"))
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

    pub fn next_line(&mut self) -> Option<String> {
        self.tokenizer.next_line()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_read() {
        let mut buffer = Cursor::new(b"-10\n1.1\n");
        assert_eq!((-10i64, 1.1f64), read!(buffer, (i64, f64)));

        let mut buffer = Cursor::new(b"-10\n1.1\n");
        assert_eq!(vec!['-', '1', '0'], read!(buffer, [char]));
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
    fn test_next_line() {
        let mut buffer: &[u8] = b"ab\r\n\nc";
        let mut tk = Tokenizer::new(&mut buffer);
        assert_eq!(tk.next_line(), Some("ab".to_string()));
        assert_eq!(tk.next_line(), Some("".to_string()));
        assert_eq!(tk.next_line(), Some("c".to_string()));
        assert_eq!(tk.next_line(), None);
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
    fn test_next_token_and_line() {
        let mut buffer: &[u8] = b"ab \nc d \n";
        let mut tk = Tokenizer::new(&mut buffer);
        assert_eq!(tk.next_token(), Some("ab".to_string()));
        assert_eq!(tk.next_line(), Some("".to_string()));
        assert_eq!(tk.next_line(), Some("c d ".to_string()));
        assert_eq!(tk.next_token(), None);
    }

    #[test]
    fn test_next_line_empty_lines() {
        let mut buffer: &[u8] = b"\n\n";
        let mut tk = Tokenizer::new(&mut buffer);
        assert_eq!(tk.next_line(), Some("".to_string()));
        assert_eq!(tk.next_line(), Some("".to_string()));
        assert_eq!(tk.next_line(), None);
    }

    #[test]
    fn test_next_token_empty_lines() {
        let mut buffer: &[u8] = b"\n\n";
        let mut tk = Tokenizer::new(&mut buffer);
        assert_eq!(tk.next_token(), None);
    }
}
