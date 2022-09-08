// Topic: Lifetimes & Functions
//
// Summary:
// Create a program that compares which string is longer (highest length).
//
// Requirements:
// * The comparison must be done using a function named `longest`
// * No data may be copied (cannot use .to_owned() or .to_string())
// * If both strings are the same length, the first one should be returned

fn main() {
    let short = "Hello";
    let long = "this is a long message";
    println!("{}", longest(short, long))
}

fn longest<'a>(first_string: &'a str, second_string: &'a str) -> &'a str{ // if there was just one parameter no need to use lifetime, since Rust knows where it is borrowed from.
    // when we use lifetime Rust knows we are returning either one of the borrowed parameter
    if second_string.len() > first_string.len() {
        second_string
    }else{
        first_string
    }
}