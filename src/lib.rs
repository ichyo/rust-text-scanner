use std::fmt::Debug;
use std::io::Read;
use std::str::FromStr;

fn is_ascii_whitespace(b: u8) -> bool {
    // Can use u8::is_ascii_whitespace once removing support of 1.15.1
    match b {
        b'\t' | b'\n' | b'\x0C' | b'\r' | b' ' => true,
        _ => false,
    }
}

pub struct Tokenizer<R: Read> {
    reader: R,
}

pub struct Scanner<R: Read> {
    tokenizer: Tokenizer<R>,
}

impl<R: Read> Tokenizer<R> {
    pub fn new(reader: R) -> Self {
        Tokenizer { reader }
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
        if let Some(b'\n') = line.last() {
            line.pop();
        }
        if let Some(b'\r') = line.last() {
            line.pop();
        }
        Some(String::from_utf8(line).expect("UTF-8 encoding error"))
    }
}

impl<R: Read> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Scanner {
            tokenizer: Tokenizer::new(reader),
        }
    }

    pub fn next_line(&mut self) -> Option<String> {
        self.tokenizer.next_line()
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
    use super::Scanner;
    use super::Tokenizer;

    #[test]
    fn test_scan() {
        let buffer: &[u8] = &[b'-', b'1', b'0', b' ', b'\n', b'1', b'.', b'1', b' ', b'\n'];
        let mut sc = Scanner::new(buffer);
        assert_eq!(sc.scan::<i64>(), Some(-10));
        assert_eq!(sc.scan::<f64>(), Some(1.1));
        assert_eq!(sc.scan::<f64>(), None);
    }

    #[test]
    fn test_next_line() {
        let buffer: &[u8] = &[b'a', b'b', b'\r', b'\n', b'\n', b'c'];
        let mut tk = Tokenizer::new(buffer);
        assert_eq!(tk.next_line(), Some("ab".to_string()));
        assert_eq!(tk.next_line(), Some("".to_string()));
        assert_eq!(tk.next_line(), Some("c".to_string()));
        assert_eq!(tk.next_line(), None);
    }

    #[test]
    fn test_next_token() {
        let buffer: &[u8] = &[b'a', b'b', b' ', b'\n', b'c', b' ', b'd', b' ', b'\n'];
        let mut tk = Tokenizer::new(buffer);
        assert_eq!(tk.next_token(), Some("ab".to_string()));
        assert_eq!(tk.next_token(), Some("c".to_string()));
        assert_eq!(tk.next_token(), Some("d".to_string()));
        assert_eq!(tk.next_token(), None);
    }

    #[test]
    fn test_next_token_and_line() {
        let buffer: &[u8] = &[b'a', b'b', b' ', b'\n', b'c', b' ', b'd', b' ', b'\n'];
        let mut tk = Tokenizer::new(buffer);
        assert_eq!(tk.next_token(), Some("ab".to_string()));
        assert_eq!(tk.next_line(), Some("".to_string()));
        assert_eq!(tk.next_line(), Some("c d ".to_string()));
        assert_eq!(tk.next_token(), None);
    }

    #[test]
    fn test_next_line_empty_lines() {
        let buffer: &[u8] = &[b'\n', b'\n'];
        let mut tk = Tokenizer::new(buffer);
        assert_eq!(tk.next_line(), Some("".to_string()));
        assert_eq!(tk.next_line(), Some("".to_string()));
        assert_eq!(tk.next_line(), None);
    }

    #[test]
    fn test_next_token_empty_lines() {
        let buffer: &[u8] = &[b'\n', b'\n'];
        let mut tk = Tokenizer::new(buffer);
        assert_eq!(tk.next_token(), None);
    }
}
