// Iterator do not do anything , they just help in configuration
// Iterator can be chained. They get executed as they get encountered in code.
fn main(){
    let numbers = vec![1, 2, 3, 4, 5];

    let plus_one: Vec<_> = numbers
    .iter()
    .map(|num| num + 1)
    .collect();

    let new_number: Vec<_> = numbers
    .iter()
    .filter(|num| num <=3)
    .collect();

    let numbers = vec![1, 2, 3, 4, 5];
    let find_num: Option<i32> = numbers
    .iter()
    .find(|num| num == 3);

    let count = numbers.iter().count();

    let last: Option<i32> = numbers.iter().last(); // reason ir returns Option is last of empty vector is None

    let min: Option<i32> = numbers.iter().min();

    let max: Option<i32> = numbers.iter().max();

    let take: Vec<i32> = numbers.iter().take(3).collect(); //vec![1, 2, 3]



}