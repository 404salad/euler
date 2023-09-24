// 1 3 5 7 9 11 13 12
extern crate prime_factorization;
use prime_factorization::Factorization;
fn main() {
    let mut i = 10000;
    loop{
        let num = (i*(i+1))/2;
        let factorsnum = factorsn(num);
        if factorsnum>500{
            println!("{num}");
            break;
        }
        i+=1
    }
}

fn factorsn(num:u128) -> u32{
    let factor_repr = Factorization::run(num);
    let mut sum = 1;
    for (_a,b) in factor_repr.prime_factor_repr(){
        sum= sum*(b+1);
    }
    sum 
    
}
/*
    let mut count = 0;
    for i in 1..num/2{
        if num%i == 0 {
            count+=1;
        }
        if (num != num/i) && ((num/i) % i == 0) {
            count+=1;
        }
    }
    count
}
*/
