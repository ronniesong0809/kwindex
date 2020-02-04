// extern crate kwindex;
use kwindex::*;

fn main() {
    let mut index = KWIndex::new();
    println!("is_empty(): {}\n", index.is_empty());

    index = index.extend_from_text("The total number of confirmed Wuhan coronavirus cases in mainland China stands at 17,205, the country's National Health Commission said Sunday, That figure is up 2,829 from the previous day. The death toll globally is 362. All but one of those deaths have occurred in China (mainland).");
    println!("\nis_empty(): {}", index.is_empty());
    println!("len(): {}", index.len());

    let check = "world";
    println!("count_matches('{}'): {}", check, index.count_matches(check));
    println!("\n{:?}", index);
}
