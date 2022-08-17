struct GroceryItem{
    stock: i32,
    price: f64,
}

fn main(){
    let spinich = GroceryItem{
        stock: 1,
        price: 5.5,
    };
    println!("stock: {:?}", spinich.stock);
    println!("price: {:?}", spinich.price);
}