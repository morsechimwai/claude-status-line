use std::io::Read;

fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().read_to_string(&mut buf);
    // Rendering wired up in later tasks.
}
