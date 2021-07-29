use std::io;
use std::collections::HashMap;

fn main() {
    let mut text = String::new();
    io::stdin().read_line(&mut text)
        .expect("error");

    let mut hash = HashMap::new();

    for word in text.chars() {
        let count = hash.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("{:?}", hash);
}
