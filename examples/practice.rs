// https://atcoder.jp/contests/practice/tasks/practice_1

use text_scanner::scan;

fn main() {
    let (a, b, c): (i32, i32, i32) = scan();
    let s: String = scan();
    println!("{} {}", a + b + c, s);
}
