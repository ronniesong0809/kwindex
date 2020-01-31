extern crate kwindex;

fn main() {
    let mut index = kwindex::KWIndex::new();
    index = index.extend_from_text("Hey world!");
    println!("len(): {}", index.len());
    println!("count_matches(): {}", index.count_matches("world"));
}
