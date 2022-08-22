enum Ticket {
    Backstage(f64, String),
    Vip(f64, String),
    Standard(f64),
}

fn main(){
    let tickets = vec! {
        Ticket::Backstage(1890.09, "Bill Gates".to_owned()),
        Ticket::Vip(999.09, "Jacky Chan".to_owned()),
        Ticket::Standard(309.99), 
    };

    for ticket in tickets{
        match ticket{
            Ticket::Backstage(price, name) => println!("Holder: {:?}, Backstage Price: {:?}", name, price),
            Ticket::Vip(price, name) => println!("Holder: {:?}, VIP Price: {:?}", name, price),
            Ticket::Standard(price) => println!("Standard Price: {:?}", price),
        }
    }
}
