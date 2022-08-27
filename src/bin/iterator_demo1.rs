fn main(){
    let data: Vec<_> = vec! [1, 2, 3, 4, 5].iter()
    .map(|num| num * 3)
    .filter(|num| num > &10)
    .collect();

    for num in data{
        println!("{:?}", num);
    }
}