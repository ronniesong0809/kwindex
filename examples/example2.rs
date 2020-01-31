extern crate kwindex;

fn main() {
    let mut index = kwindex::KWIndex::new();
    index = index.extend_from_text("The total number of confirmed Wuhan coronavirus cases in mainland China stands at 17,205, the country's National Health Commission said Sunday, That figure is up 2,829 from the previous day. The death toll globally is 362. All but one of those deaths have occurred in mainland China.");
    println!("len(): {}", index.len());
    println!("count_matches(): {}", index.count_matches("world"));
    println!("{:?}", index);
}
