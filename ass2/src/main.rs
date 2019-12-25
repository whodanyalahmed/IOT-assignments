// use std::io;

// // Question 1

// fn main() {
//     loop {
//     println!("Calculator");
//     println!("==========");
//     println!("1 - Addittion");
//     println!("2 - Subtract");
//     println!("3 - Divide");
//     println!("4 - Multiply");
//     println!("5 - Power");
//     println!("6 - Quit");
    
//     println!("operator: ");
//     let mut o = String::new();
//     std::io::stdin().read_line(&mut o);
//     let o:char = o.trim().parse().unwrap();
    
    


//     let mut a = String::new();
//     let mut b = String::new();

//     if (o ==  '1' ){
//     println!("Enter value: ");
//     std::io::stdin().read_line(&mut a);
//     let a:f32 = a.trim().parse().unwrap();
//     println!("Enter second value: ");
//     std::io::stdin().read_line(&mut b);
//     let b:f32 = b.trim().parse().unwrap();
    
//         println!("The answer is {}",(a+b) );
//     }
//     else if (o ==  '2' ){
//         println!("Enter value: ");
//         std::io::stdin().read_line(&mut a);
//         let a:f32 = a.trim().parse().unwrap();
//         println!("Enter second value: ");
//         std::io::stdin().read_line(&mut b);
//         let b:f32 = b.trim().parse().unwrap();
        
//         println!("The answer is {}",(a-b) );
//     }
//     else if (o ==  '3' ){
//         println!("Enter value: ");
//         std::io::stdin().read_line(&mut a);
//         let a:f32 = a.trim().parse().unwrap();
//         println!("Enter second value: ");
//         std::io::stdin().read_line(&mut b);
//         let b:f32 = b.trim().parse().unwrap();
        
//         println!("The answer is {}",(a/b) );
//     }
//     else if (o ==  '4' ){
//         println!("Enter value: ");
//         std::io::stdin().read_line(&mut a);
//         let a:f32 = a.trim().parse().unwrap();
//         println!("Enter second value: ");
//         std::io::stdin().read_line(&mut b);
//         let b:f32 = b.trim().parse().unwrap();
        
//         println!("The answer is {}",(a*b) );
//     }

//     else if (o ==  '5' ){
//         println!("Enter value: ");
//         std::io::stdin().read_line(&mut a);
//         let a:f32 = a.trim().parse().unwrap();
//         println!("Enter second value: ");
//         std::io::stdin().read_line(&mut b);
//         let b:f32 = b.trim().parse().unwrap();
        
//         let a = a as u32;
//         let b= b as u32;
//         println!("The answer is {}",pow(a,b));
//     }
//     else if(o == '6'){
//         println!("Quiting calculator.....");
//         break;
//     }
//     else{
//         println!("Error finding operator...");
//     }
// }
// }
// fn pow(mut a :u32 ,b: u32 ) -> u32{
//     let mut c = a;
//     for i in 1..b {
//         c*= a
//     }
//     c
// }


// Question 2 

// fn main() {
//     let x =|| println!("Hello! world..."); //make a closure which takes no argument and prints hello world
//     x();
// }
// fn main() {
//     let x = |x:u32| x+1;//Make a closure which takes one u32 data type as argument and returns with adding 1 to it.
//     let y = 6;
//     println!("The function returns: {}",x(y)); 
// }
// fn main() {
//     let mut c = 1;
//     let mut x = || c = c +1;  //Make a closure which captures value of variable "c" from environment and change the value of c with adding 1.
//     x();
//     println!("The new value of c is: {}",c); // should print 2
// }
// Write a function which accepts a closure, and in the funciton body, it calls the closure. The closure accepts no argument and returns nothing. What should the closure be about? Be creative!
// fn main() {
//     let closure = || println!("");
//     nothing(closure);
// }


// fn nothing<T: Fn() > (x:T){
//     x();
// }

// Write a function which expects a closure as an argument and in the funciton body, it calls the closure. The closure expects u32 argument and returns the u32 value. The closure adds 1 to the argument and returns it.
fn main() {
    let d:u32 = 4;
    let closure = |c| c + 1 ;
    nothing(closure(d));
}


fn nothing<T: Fn() > (x:T){
    x();
}

