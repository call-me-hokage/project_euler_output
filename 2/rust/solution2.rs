
fn main () {
    let  limit = 400000;
    let  (mut a, mut b) = (1,1);
    let mut total = 0;
    loop {
        let c = a+b;
        if c > limit {
            break;
        }
        if c % 2 == 0 {
            total +=c;
        }
        a = b;
        b = c;
    }    
    println!("{}",total);

}
