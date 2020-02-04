extern crate kwindex;

fn main() {
    let mut index = kwindex::KWIndex::new();
    println!("is_empty(): {}", index.is_empty());
    index = index.extend_from_text("Hey world!");
    println!("is_empty(): {}", index.is_empty());
    println!("len(): {}", index.len());
    println!("count_matches(): {}", index.count_matches("world"));
    println!("{:?}", index);
}
