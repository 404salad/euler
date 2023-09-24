fn main() {
    let mut full = String::new();
    for i in 0..=1000_000 {
        full.push_str(&i.to_string());    
    }
    
    println!("{}",full.chars().nth(1).unwrap());
    println!("{}",full.chars().nth(10).unwrap());
    println!("{}",full.chars().nth(100).unwrap());
    println!("{}",full.chars().nth(1000).unwrap());
    println!("{}",full.chars().nth(10000).unwrap());
    println!("{}",full.chars().nth(100000).unwrap());
    println!("{}",full.chars().nth(1000_000).unwrap());
}

