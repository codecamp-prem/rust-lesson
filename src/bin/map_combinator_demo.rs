//Map combinator is used to transform one value to another
#[derive(Debug)]
struct User{
    user_id: i32,
    name: String,
}

/// Locates a user_id based on the name.
fn find_user(name: &str) -> Option<i32>{
    let name = name.to_lowercase();
    match name.as_str() {
        "john" => Some(1),
        "jane" => Some(5),
        "kate" => Some(9),
        _ => None,
    }
}

fn main(){
    let user_name = "Kate";
    let user = find_user(user_name)
        .map(|user_id| User{ //Option type has .map implemented on it.
            user_id, 
            name: user_name.to_owned()
        }
    );
    match user{
        Some(user) => println!("{:?}", user),
        None => println!("No User Found!"),
    }
}