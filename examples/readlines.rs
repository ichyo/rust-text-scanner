extern crate text_scanner;

use std::error::Error;
use text_scanner::*;

fn run() -> Result<(), Box<Error>> {
    let mut output = 0i64;
    let v: Vec<i64> = scan_iter().collect::<Result<_, _>>()?;
    for x in v {
        output += x;
    }
    println!("{}", output);
    Ok(())
}

fn main() {
    run().unwrap();
}
