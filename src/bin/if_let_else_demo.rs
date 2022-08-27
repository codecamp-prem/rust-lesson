enum Color{
    Red, 
    Green,
    Blue,
}

fn main(){
    let maybe_user = Some("John");
    if let Some(user)=maybe_user{
        println!("user: {:?}", user);
    }else{
        // if you use else, it is better to use match
        println!("no user");
    }

    let blue = Color::Blue;
    if let Color::Blue = blue{
        println!("It's Blue");
    }else{
        // if you use this else, it is better to use match expression
        println!("It's not Blue");
    }
}