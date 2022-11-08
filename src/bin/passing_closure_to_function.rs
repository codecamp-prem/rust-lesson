fn math(a: i32, b:i32, op: Box<dyn Fn(i32, i32) ->i32>) -> i32{
    op(a, b)
}
fn main(){
    let add = |a, b| a+b; //creating closure named add
    let add: Box<_> = Box::new(add); // saving a closure into a Box - closure size can vary so we are keeping it in Box
    println!("Add: {}", math(9, 3, add));
    let something = "Rust";
    let sub = Box::new(move |a, b| { // we have to take ownership when we box "something" so we need to add move, without Box we don't need to write move.
        println!("Closure will enclosed the environment, so everything define before the closure will enter into the closure, that's why we will get somthing inside this closure too : {}", something); //Note that something is not accessible outside now, since we have moved it.
        a-b
    });
    let mul = Box::new(|a, b| a*b);
    println!("Sub: {}", math(9, 3, sub));
    println!("Mul: {}", math(9, 3, mul));
}