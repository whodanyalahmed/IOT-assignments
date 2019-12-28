fn main() {
    // let closure = || println!("This is Dan");
    // nothing(closure);
    // nothing1(closure);
}

// Quesdtion 1
fn nothing<T: FnOnce() > (mut x:T){
    x();
}
// Question 2
fn nothing1<T: FnMut () > (mut x:T){
    x();
}


// Question 3
use std::thread;
use std::time::Duration;
fn main() {
    let handle = thread::spawn(||{
      for i in 1..10{
            println!("executing from i: {}",i);
            thread::sleep(Duration::from_millis(1));
        }  
    });
    
    handle.join().unwrap();
    let handle2 = thread::spawn(||{
      for f in 1..8{
            println!("executing from f: {}",f);
            thread::sleep(Duration::from_millis(1));
        }  
    });
    handle2.join().unwrap();
    
    for j in 1..5{
        println!("executing from j: {}",j);
        thread::sleep(Duration::from_millis(1));
    }
    
    
    
}
