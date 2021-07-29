use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        text: String,
    }

    let mut hash = HashMap::new();

    for word in text.chars() {
        let count = hash.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("{:?}", hash);
}
