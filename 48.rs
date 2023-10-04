

fn main() {
    println!("whta");
    let mut sum:u128 = 0;
    for i in 1..10 {
        for j in 1..10 {
            println!("{}", (i as u128).pow(j as u32));
        }
    }
}

