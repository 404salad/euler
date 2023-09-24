#![allow(unused)]
fn main() {
    let maxp: u32= 1000;
    let mut maxans = 0;
    let mut ans = 0;
    // a^2 + b^2 = c^2
    // a + b < c
    // a + b + c = p
    for p in 2..maxp {
        let mut trianglenumbers = Vec::new();
        for a in 1..p{
            for b in 1..p/2{
                if a+b < p{
                    let c = (p - a - b) as f64;
                    if c == ((a.pow(2) + b.pow(2)) as f64).sqrt(){
                    if !trianglenumbers.contains(&a) && !trianglenumbers.contains(&b){
                        trianglenumbers.push(a);
                        }
                    }
                }
            }
        }
        if maxans<trianglenumbers.len(){
            maxans = trianglenumbers.len();
            ans = p;
        }
    }
    println!("{ans}");
}
