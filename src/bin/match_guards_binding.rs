enum Status{
    Error(i32),
    Info,
    Warn,
}

fn main(){
    let status = Status::Error(4);
    match status{
        Status::Error(s @ 3) => println!("error three"),
        Status::Error(s @ 5..=6) => println!("error 5 or 6: {}", s),
        Status::Error(s @ 7..=11) => println!("error 7 through 11: {}", s),
        Status::Error(s @ 12 | s @ 14) => println!("error 12 or 14"),
        Status::Error(s) => println!("error code: {}", s),
        Status::Info => println!("Info"),
        Status::Warn => println!("Warn"),
    }
}