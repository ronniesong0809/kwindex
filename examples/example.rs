extern crate kwindex;

fn main() {
    let index = kwindex::KWIndex::new();
    print!("{}", index.len());
    print!("{}", index.count_matches("world"));
}