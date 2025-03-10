#![allow(unused)]
fn is_prime(n: u32) -> bool {
    let mut returner: bool = true;
    for i in 2..((n as f32).sqrt() as u32) {
        if n % i == 0 {
            returner = false;
        }
    }
    returner
}
fn is_pandigital(n: u32) -> bool {
    let mut mutn = n;
    let mut returner: bool = true;
    let nstr = n.to_string();
    let nlen = nstr.len();
    let mut sumi: u32 = 0;
    let mut sumd: u32 = 0;
    for i in 1..=nlen {
        let digit = mutn % 10;
        mutn = mutn / 10;
        sumi += i as u32;
        sumd += digit;
    }
    if sumi != sumd {
        returner = false;
    }
    returner
}
fn main() {
    for i in (1..987_654_322).rev() {
        if is_pandigital(i) {
            if is_prime(i) {
                println!("{i}");
                break;
            }
        }
    }
}
