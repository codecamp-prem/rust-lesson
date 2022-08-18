fn main(){
    let coord = (45.04, 67.76);
    println!("{:?}, {:?}", coord.0, coord.1);

    // destructure, recommended 
    let (x, y) = coord;
    println!("{:?}, {:?}", x, y);

    // can be of different data type
    let user_info = ("Jane Doe", 32);
    let (name, age) = user_info;
    println!("{:?}, {:?}", name, age);
}