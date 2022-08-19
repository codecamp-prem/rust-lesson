struct Grocery{
    id: i32,
    quantity: i32,
}

fn display_id(grocery: &Grocery){
    println!("ID: {:?}", grocery.id);
}

fn display_quantity(grocery: &Grocery){
    println!("Quantity: {:?}", grocery.quantity);
}

fn main(){
    let item = Grocery{
        id: 1,
        quantity: 1000,
    };
    display_id(&item);
    display_quantity(&item);
}