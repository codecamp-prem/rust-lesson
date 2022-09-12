enum Species {
    Finch,
    Hawk,
    Parrot,
}

struct Bird{
    age: usize,
    species: Species,
}

fn main(){
    let hawk = Bird{
        age: 17,
        species: Species::Hawk,
    };
    //Matching Structure
    match hawk{
        Bird{ age: 4, .. } => println!("4 years old bird"), // .. ignore everthing else in this context
        Bird{ age: 4..=10 | 15..=20, .. } => println!("4-10 or 15-20 year old bird"),
        Bird{ species: Species::Parrot, .. } => println!("Parrot!"),
        Bird{ .. } => println!("Other Bird") //.. indicates everthing else in this context
    }
}