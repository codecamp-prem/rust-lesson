fn main(){
    let num = vec! [1, 2, 3];
    match num.is_empty() {
        true => println!("Vector is empty"),
        false => println!("Vector is NOT empty")
    }
}