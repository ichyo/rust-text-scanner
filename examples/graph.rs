use text_scanner::{scan, scan_iter};

fn main() {
    let (n, m): (usize, usize) = scan();
    let mut graph = vec![Vec::new(); n];
    for (u, v) in scan_iter::<(usize, usize)>().take(m) {
        graph[u].push(v);
        graph[v].push(u);
    }
}
