enum Discount{
    Percent(i32),
    Flat(i32),
}

struct Ticket{
    event: String,
    price: i32,
}

fn main(){
    let x = 3;
    match x {
        3 => println!("three"),
        other => println!("{:?}", other),
    }

    let flat = Discount::Flat(2);
    match flat{
        Discount::Flat(2) => println!("Flat 2"),
        Discount::Flat(amount) => println!("flat discount of {:?}", amount),
        _ => (), //ignore Percent varient in enum
    }

    let concert = Ticket{
        event: "World-Tour".to_owned(),
        price: 77,
    };

    match concert {
        Ticket{price: 77, event} => println!("{:?} @ 77", event),
        Ticket{price, ..} => println!("price: {:?}", price), // .. means ignore other fields and get the price if it is not 77
    }
}