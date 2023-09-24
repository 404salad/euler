#![allow(unused)]
fn main(){
    let mut productnum = 1;
    let mut productden = 1;
    // number1/number2 
    for tens1 in 1..=9{
    for units1 in 1..=9{
    for tens2 in 1..=9{
    for units2 in 1..=9{
           let number1 = tens1 * 10 + units1;
           let number2 = tens2 * 10 + units2;
           let mut naivefraction:f64 = 0.0 ;
           if number1 < number2{
               let normalfraction = number1 as f64 / number2 as f64;
               if tens1 == units2 {
                   naivefraction =  units1 as f64/tens2 as f64;
               }
               else if units1 == tens2 {
                   naivefraction = tens1 as f64/units2 as f64;
               }
               if naivefraction == normalfraction{
                   println!("{number1}/ {number2}");
                   productnum *= number1;
                   productden *= number2;
               }
           }
    }}}}
    println!("{productnum}/ {productden}");
}
