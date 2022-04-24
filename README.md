# rust-text-scanner
A simple text scanner to parse primitive types with no dependency for competitive programming 

The minimum supported compiler version is 1.31.0.

## Usage

```toml
[dependencies]
text-scanner = { git = "https://github.com/ichyo/rust-text-scanner" }
```


## Examples

### Read three integers and one string.

```rust
use text_scanner::scan;

fn main() {
    let (a, b, c): (i32, i32, i32) = scan();
    let s: String = scan();
    println!("{} {}", a + b + c, s);
}
```

### Read n integers and output sum.

```rust
use text_scanner::{scan, scan_iter};

fn main() {
    let n: usize = scan();
    let sum: i64 = scan_iter::<i64>().take(n).sum();
    println!("{}", sum);
}
```

### Read edges (0-index) and construct adjacent list

```rust
use text_scanner::{scan, scan_iter};

fn main() {
    let (n, m): (usize, usize) = scan();
    let mut graph = vec![Vec::new(); n];
    for (u, v) in scan_iter::<(usize, usize)>().take(m) {
        graph[u].push(v);
        graph[v].push(u);
    }
}
```
