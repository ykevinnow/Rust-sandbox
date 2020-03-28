// 
use std::collections::HashMap;

pub fn run() {
    let mut hm = HashMap::new();

    hm.insert(String::from("random"), 12);
    hm.insert(String::from("strings"), 49);

    for (k, v) in &hm {
        println!("{}: {}", k, v);
    }

    match hm.get(&String::from("random")) {
        Some(&n) => println!("{}", n),
        _ => println!("no match"),
    }

    hm.remove(&String::from("strings"));
}