// Create outside of main function
enum Direction {
        Up,
        Down, 
        Right,
        Left
    }
fn main(){
    let go = Direction::Left;
    match go {
        Direction::Up => println!("Go Up"),
        Direction::Down => println!("Go Down"),
        Direction::Right => println!("Go Right"),
        Direction::Left => println!("Go Left"),
    }
}