enum Access {
    Full,
}

fn return_three_value() -> (i32, i32, i32){
    (5, 6, 7)
}

fn main(){
    let numbers = return_three_value();
    let (x, y, z) = return_three_value();
    println!("{:?}, {:?}", x, numbers.0); //5
    println!("{:?}, {:?}", y, numbers.1); //6
    println!("{:?}, {:?}", z, numbers.2); //7

    let (employee, access) = ("Harry", Access::Full);
}