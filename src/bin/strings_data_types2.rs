struct Person{
    name: String,
    age: i32,
    color: String,
}

fn print_data(data: &str){
    println!("{:?}", data);
}

fn main(){
    let users = vec! {
        Person {
            name: String::from("Tom"),
            age: 10,
            color: "Red".to_string(),
        },
        Person {
            name: "Dick".to_string(),
            age: 7,
            color: String::from("Green"),
        },
        Person {
            name: String::from("Harry"),
            age: 34,
            color: "Blue".to_string(),
        }
    };
    for person in users{
        if person.age <= 10{
            print_data(&person.name);
            println!("Age: {:?}", person.age);
            print_data(&person.color);
        }
    }
}