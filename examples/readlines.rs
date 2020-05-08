extern crate text_scanner;

use std::error::Error;
use text_scanner::*;

fn run() -> Result<(), Box<dyn Error>> {
    let mut output = 0i64;
    let v: Vec<i64> = scan_iter().collect();
    for x in v {
        output += x;
    }
    println!("{}", output);
    Ok(())
}

fn main() {
    run().unwrap();
}
