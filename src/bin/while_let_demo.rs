fn main(){
    // while let may be useful for iterator who returns OPTIONAL data
    let numbers = vec! [1, 2, 3, 4, 5];
    let mut num_iter = numbers.iter();
    while let Some(num) = num_iter.next() {
        println!("{:?}", num);
    }
}