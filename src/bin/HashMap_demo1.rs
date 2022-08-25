use std::collections::HashMap;

fn main(){
    let mut store = HashMap::new();
    store.insert("Chair", 5);
    store.insert("Bed", 3);
    store.insert("Table", 5);
    store.insert("Couch", 0);
    
    let mut total_stock = 0;

    for (furniture, qty) in store.iter(){
        total_stock = total_stock + qty;
         
        let stock = if qty == &0 { //iter is borrowed so 0 should be also borrow. &0
            "Out of Stock".to_owned()
        }else{
            format!("{:?}", qty)
        };

        println!("{:?} : {:?}", furniture, stock);
    }

    println!("Total Stock: {:?}", total_stock);
}