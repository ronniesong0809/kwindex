extern crate kwindex;
use kwindex::*;

fn main() {
    let mut index = KWIndex::new();
    println!("is_empty(): {}\n", index.is_empty());

    index = index.extend_from_text("Hey world!");
    println!("\nis_empty(): {}", index.is_empty());
    println!("len(): {}", index.len());

    let check = "world";
    println!("count_matches('{}'): {}", check, index.count_matches(check));
    println!("\n{:?}", index);
}
