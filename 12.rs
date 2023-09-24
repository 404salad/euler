// 1 3 5 7 9 11 13 12
extern crate prime_factorization;
use prime_factorization::Factorization;
fn main() {
    let mut i:u128 =1779441;
    let mut j:u128 = 2;
    loop{
        let factorsnum = factorsn(i);
        println!("{i} {factorsnum}");
        if factorsnum>300{
            break;
        }
        i+=j; 
        j+=1;
    }
}

fn factorsn(num:u128) -> u128{
    let factor_repr = Factorization::run(num);

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
