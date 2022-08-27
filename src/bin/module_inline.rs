mod greet{
    pub fn hello(){ // public function can be accessed
        println!("Hello!!");
    }

    fn goodbye(){ //private function cannot be accessed 
        println!("Bye Bye!!");
    }
}

mod math {
    fn add(a: i32, b: i32) -> i32{
        a + b
    }

    pub fn multiply(a: i32, b: i32) -> i32{ // public fn
        a * b 
    }
}

fn main(){
    use greet::hello; // or use greet::*;
    hello();
    println!("{:?}",math::multiply(2, 9));
}