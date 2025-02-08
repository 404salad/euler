fn main() {
    println!("whta");
    let mut sum: u128 = 0;
    let mut product: u128 = 1;
    for i in 1..1000 {
        for j in 0..i {
            product = product * i;
            if product > 100000_000_000 {
                product %= 100000_000_000
            }
        }
        sum += product;
        if sum > 100000_000_000 {
            sum %= 100000_000_000
        }
    }
    println!("{}", sum);
    // 8945256360
}
