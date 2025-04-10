fn main() {
    let mut number:u64 = 600851475143;
    let mut factor = 2;
    
    while factor * factor <= number{
        if number % factor ==0{
            number /=factor;
        } else {
            factor +=1;
        }
            
    }
    println!("The largest prime factor is: {}", number);
}
