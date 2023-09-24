fn main() {
    let mut answer = 0;
    for n in 1..=10000000{
        let mut tempnum = n;
        let mut sum = 0;
        while tempnum!=0{
            let digit = tempnum%10;
            tempnum = tempnum/10;
            let square = (digit as u128).pow(5);       
            sum += square;
        }
        if n == sum{
            println!("{n} {sum}");
            answer += sum;
        }

    } 
    println!("{answer}");
}

