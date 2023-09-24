fn main() {
    let _coins = [1,2,5,10,20,50,100,200];
    let multiplier = [200,100,40,20,10,4,2,1];
    let goal = 200;
    let mut count = 8;// individual coins
    let mut sum;
    for i in 0..multiplier[0]{
    for j in 0..multiplier[1]{
    for k in 0..multiplier[2]{
    for l in 0..multiplier[3]{
    for m in 0..multiplier[4]{
    for n in 0..multiplier[5]{
    for o in 0..multiplier[6]{
       sum = i*1+j*2+k*5+l*10+m*20+n*50+o*100;
       if sum == goal{
           count+=1;
       }
    }}}}}}}
println!("{count}");
}
