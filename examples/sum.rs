use text_scanner::{scan, scan_iter};

fn main() {
    let n: usize = scan();
    let sum: i64 = scan_iter::<i64>().take(n).sum();
    println!("{}", sum);
}
