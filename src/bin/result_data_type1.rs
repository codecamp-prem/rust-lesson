struct Customer{
    age: i32,
}

fn age_restricted(input: &Customer)-> Result<(), String> {
    if input.age < 21 {
        Err("Customer Must be atleast 21 years of age.".to_owned())
    }else{
        Ok(())
    }
}

fn main(){
    let customer = Customer{
        age: 29
    };
    let purchase = age_restricted(&customer);
    println!("{:?}", purchase);
}