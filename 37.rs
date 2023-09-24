#![allow(unused)]
fn checkprime(num:u128)->bool {
    let mut returner = true;
    for i in 2..=(((num as f64).sqrt()) as u128){
        if num%i == 0 {returner = false;}
    } 
    if num ==1{
        returner = false;
    }
    returner
}

fn rightshift(num:u128)->u128{
    num/10
}

fn leftshift(num:u128)->u128{
    let mut tempnum = num;
    let mut digitlen = 0;
    let mut returnnum = 0;
    //for digitlen
    while tempnum!=0 {
        tempnum/=10;
        digitlen+=1;
    }
    tempnum = num;
    //for reversing the number
    for i in 1..digitlen{
        let digit = tempnum %10;
        returnnum += digit * 10_u128.pow(i as u32);
        tempnum = tempnum/10;
    } 
    returnnum/10
}

fn istprime(num:u128)->bool {
    let mut leftnum = num;
    let mut rightnum = num;
    let mut returner = true;
    while leftnum!=0{
        if checkprime(leftnum) {} else {returner = false};
        leftnum = leftshift(leftnum);
    }
    while rightnum!=0{
        if checkprime(rightnum) {} else {returner = false};
        rightnum = rightshift(rightnum);
    }
returner
}

fn main() {
    let mut n = 9;
    let mut ans = 0;
    while n<1000000{
       if checkprime(n) {
          if istprime(n) {ans += n}
       }
       n+=2;
   } 
    println!("{ans}");
}
