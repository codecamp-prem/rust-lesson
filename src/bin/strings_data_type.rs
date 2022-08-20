fn print_it(data: &str){
    println!("{:?}", data);
}

fn main(){
    print_it("borrowed when created this way");
    let owned_string = "owned string".to_owned();
    let another_owned = String::from("another owned string");
    print_it(&owned_string); //use &<str> when passing to the function
    print_it(&another_owned);
}