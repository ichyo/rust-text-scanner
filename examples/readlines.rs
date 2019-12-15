use std::error::Error;
use text_scanner::sscan;
use text_scanner::*;

fn main() -> Result<(), Box<Error>> {
    let mut output = 0i64;
    while let Some(s) = readln()? {
        let v: Vec<i64> = sscan!(s, [i64])?;
        for x in v {
            output += x;
        }
    }
    println!("{}", output);
    Ok(())
}
