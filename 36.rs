

fn palindrome(num: u64)-> bool{
    let mut tempnum = num;
    let mut revnum = 0;
    let mut digit;
    let base:u64= 10;
    let numlen:u64 = num.to_string().len() as u64;
    let mut i:u64 = 0;

    while i!=numlen{
        digit = tempnum%base;//remainder
        let power = numlen - i - 1;
        revnum += digit * base.pow(power as u32);
        tempnum= tempnum/10;
        i += 1;
        }
    if revnum == num{true}
    else {false}
}


fn main() {
    let mut sum:u64 = 0;
    for i in 1..=1000_000{
        let a:u64 = format!("{i:b}").parse().unwrap();
        if palindrome(i) && palindrome(a){
            sum+=i as u64;
        }                      
    }
    println!("{sum}");
}
