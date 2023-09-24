fn factorial(num:u128) -> u128 { 
    let mut product = 1; 
    for i in 1..=num{
        product*=i;
    }
    product
}

fn summer(num:u128) -> u128 {
    let mut tempnum = num;
    let mut digit = 0;
    let mut sum = 0;
    while tempnum!=0{
        digit = tempnum%10;
        tempnum = tempnum/10;
        sum+=factorial(digit);
    }
    sum
}


fn main() {
   for i in 10..=99999999{
       if i == summer(i){
           println!("{i}");
       }
   }
}
