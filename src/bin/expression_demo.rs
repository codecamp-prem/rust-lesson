fn main(){
    let compare_value = 99;
    let value = 100;
    let is_gt_value = value > compare_value;
    print_msg(is_gt_value, value, compare_value)
}

fn print_msg(gt_value: bool, value: i32, compare_value: i32){
    match gt_value{
        true => println!("{:?} is greater than {:?}", value, compare_value),
        false => println!("{:?} is smaller than {:?}", value, compare_value),
    }
}