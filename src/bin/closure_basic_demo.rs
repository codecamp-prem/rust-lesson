fn main(){
    let add = |a: i32, b: i32| -> i32 {
        a + b
    };
    // or short-hand
    let add = |a, b| a + b;

    let sum = add(1, 2);
}