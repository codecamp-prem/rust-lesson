use std::collections::HashMap;

#[derive(Debug)]
struct Contents{
    content: String,
}

fn main(){
    let mut lockers = HashMap::new();
    lockers.insert(1, Contents{ content: "stuff".to_owned() });
    lockers.insert(2, Contents{ content: "shoes".to_owned() });
    lockers.insert(3, Contents{ content: "money".to_owned() });

    for (locker_num, content) in lockers.iter(){
        println!("locker no. = {:?}, content={:?}", locker_num, content);
    }
}