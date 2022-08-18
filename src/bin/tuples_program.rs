fn coordinate()->(i32,i32){
    (1, 5)
}

fn main(){
    let (x, y) = coordinate();
    if y > 5 {
        println!("Greater than 5");
    }else if y < 5 {
        println!("Less than 5");
    }else{
        println!("Equal to 5");
    }
}