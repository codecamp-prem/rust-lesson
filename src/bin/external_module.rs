// Topic: External Modules

// Requirements:
// * Organize the code into two external modules based on their functionality:
//   - msg: string formatting functions
//   - math: math functions
// * Update the main function to use the functionality from the modules
//
// Notes:
// * Update your Cargo.toml to include a library file
// * After moving the functions into modules, try running
//   `cargo check --bin a26c` to get a listing of required code changes

fn main() {
    use activity::math;
    // Part 1: math functions
    let result = {
        let two_plus_two = math::add(2, 2);
        let three = math::sub(two_plus_two, 1);
        math::mul(three, three)
    };

    // Ensure we have a correct result.
    assert_eq!(result, 9);
    println!("(2 + 2 - 1) * 3 = {}", result);

    { // this curly brace creates a block, so this no pollute other code
        use activity::msg::{trim, capitalize, exciting};
        // Part 2: string functions
        let hello = {
            let msg = "hello ";
            let msg = trim(msg);
            capitalize(msg)
        };
        let world = {
            let msg = "world";
            exciting(msg)
        };
        let msg = format!("{}, {}", hello, world);

        // Ensure we have a correct result.
        assert_eq!(&msg, "Hello, world!");
        println!("{}", msg);
    }
}
