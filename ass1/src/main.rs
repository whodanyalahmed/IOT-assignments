#[warn(unused_imports)]
use std::io;

fn main() {
    println!("Enter value: ");
    let mut a = String::new();
    std::io::stdin().read_line(&mut a);
    let a:f32 = a.trim().parse().unwrap();
    
    println!("Enter second value: ");
    let mut b = String::new();
    std::io::stdin().read_line(&mut b);
    let b:f32 = b.trim().parse().unwrap();
    
    println!("operator: ");
    let mut o = String::new();
    std::io::stdin().read_line(&mut o);
    let o:char = o.trim().parse().unwrap();
    
    if (o ==  '+' ){
        println!("{}",(a+b) );
    }
    else if (o ==  '-' ){
        println!("{}",(a-b) );
    }
    else if (o ==  '/' ){
        println!("{}",(a/b) );
    }
    else if (o ==  '*' ){
        println!("{}",(a*b) );
    }
    else{
        println!("Error finding operator...");
    }




}
