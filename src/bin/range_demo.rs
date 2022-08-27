fn main(){
    let range = 1..=4; // will include last value 4
    let range = 1..4; // will not include last value 4

    for num in 1..4{
        println!("{:?}", num); // 1, 2, 3
    }

    for ch in 'a'..='f' { // = means including 'f'
        println!("{:?}", ch); // 'a', 'b', 'c', 'd', 'e', 'f'
    }
}