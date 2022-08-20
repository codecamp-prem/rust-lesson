struct LineItem{
    name: String,
    count: i32,
}

fn print_name(name: &str){
    // when passing string should be borrowed
    println!("name: {:?}", name);
}

fn main(){
    let receipt = vec! {
        LineItem{
            name: "Hard Drinks".to_owned(),
            count: 6,
        },
        LineItem{
            name: String::from("meat"),
            count: 2,
        },
    };
    for item in receipt{
        print_name(&item.name);
        println!("count:{:?}", item.count);
    }
}