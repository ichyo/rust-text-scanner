// to support 1.15.1
#![allow(unknown_lints)]
#![allow(renamed_and_removed_lints)]
#![allow(redundant_field_names)]

use std::fmt::{Debug, Display};
use std::io::{self, BufRead, Read};
use std::str::FromStr;
use std::string::FromUtf8Error;

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
        $r.scan_chars()
    };
    ($r:expr, [$t:ty]) => {
        $r.scan_all::<$t>()
    };
    ($r:expr, [$t:ty; $n:expr]) => {
        $r.scan_vec::<$t>($n)
    };
    ($r:expr, $t:ty) => {
        $r.scan::<$t>()
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

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    EncodingError(FromUtf8Error),
    ParseError(String),
    Eof,
}

impl<'a, R: Read> Tokenizer<'a, R> {
    pub fn new(reader: &'a mut R) -> Self {
        Tokenizer { reader: reader }
    }

    pub fn next_token(&mut self) -> Result<Option<String>, Error> {
        let mut token = Vec::new();
        for b in self.reader.by_ref().bytes() {
            let b = b.map_err(Error::IoError)?;
            match (is_ascii_whitespace(b), token.is_empty()) {
                (false, _) => token.push(b),
                (true, false) => break,
                (true, true) => {}
            }
        }
        if token.is_empty() {
            return Ok(None);
        }
        String::from_utf8(token)
            .map(Some)
            .map_err(Error::EncodingError)
    }
}

impl<'a, R: Read> Scanner<'a, R> {
    pub fn new(reader: &'a mut R) -> Self {
        Scanner {
            tokenizer: Tokenizer::new(reader),
        }
    }

    pub fn scan<T>(&mut self) -> Result<T, Error>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug + Display,
    {
        let token = self.tokenizer.next_token()?;
        match token {
            Some(s) => s
                .parse::<T>()
                .map_err(|e| Error::ParseError(format!("{}", e))),
            None => Err(Error::Eof),
        }
    }

    pub fn scan_chars(&mut self) -> Result<Vec<char>, Error> {
        Ok(self.scan::<String>()?.chars().collect())
    }

    pub fn scan_all<T>(&mut self) -> Result<Vec<T>, Error>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug + Display,
    {
        let mut result = Vec::new();
        loop {
            let s = self.tokenizer.next_token()?;
            let s = match s {
                Some(s) => s
                    .parse::<T>()
                    .map_err(|e| Error::ParseError(format!("{}", e)))?,
                None => break,
            };
            result.push(s);
        }
        Ok(result)
    }

    pub fn scan_vec<T>(&mut self, n: usize) -> Result<Vec<T>, Error>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug + Display,
    {
        let mut result = Vec::new();
        for _ in 0..n {
            let s = self.tokenizer.next_token()?;
            let s = match s {
                Some(s) => s
                    .parse::<T>()
                    .map_err(|e| Error::ParseError(format!("{}", e)))?,
                None => return Err(Error::Eof),
            };
            result.push(s);
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_read() {
        let mut buffer = Cursor::new(b"-10\n1.1\n");
        assert_eq!(-10i64, fscan!(buffer, i64).unwrap());
        assert_eq!(1.1f64, fscan!(buffer, f64).unwrap());

        let mut buffer = Cursor::new(b"-10\n1.1\n");
        assert_eq!((-10i64, 1.1f64), fscan!(buffer, (i64, f64)).unwrap());

        let mut buffer = Cursor::new(b"-10\n1.1\n");
        assert_eq!(
            vec![(-10i64, 1.1f64)],
            fscan!(buffer, [(i64, f64); 1]).unwrap()
        );

        let mut buffer = Cursor::new(b"-10\n1.1\n");
        assert_eq!(vec![-10f64, 1.1f64], fscan!(buffer, [f64; 2]).unwrap());

        let mut buffer = Cursor::new(b"-10\n1.1\n");
        assert_eq!(vec!['-', '1', '0'], fscan!(buffer, [char]).unwrap());
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
        assert_eq!(sc.scan::<i64>().unwrap(), Some(-10));
        assert_eq!(sc.scan::<f64>().unwrap(), Some(1.1));
        assert_eq!(sc.scan::<f64>().unwrap(), None);
    }

    #[test]
    fn test_next_token() {
        let mut buffer: &[u8] = b"ab \nc d \n";
        let mut tk = Tokenizer::new(&mut buffer);
        assert_eq!(tk.next_token().unwrap(), Some("ab".to_string()));
        assert_eq!(tk.next_token().unwrap(), Some("c".to_string()));
        assert_eq!(tk.next_token().unwrap(), Some("d".to_string()));
        assert_eq!(tk.next_token().unwrap(), None);
    }

    #[test]
    fn test_next_token_empty_lines() {
        let mut buffer: &[u8] = b"\n\n";
        let mut tk = Tokenizer::new(&mut buffer);
        assert_eq!(tk.next_token().unwrap(), None);
    }
}
