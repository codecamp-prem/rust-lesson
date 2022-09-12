#[derive(PartialEq, PartialOrd, Eq, Ord)]
enum Difficulty{
    Easy,
    Normal,
    Hard,
}

fn main(){
    let stage = 5;
    let difficulty = Difficulty::Normal;
    // match with if and binding @
    match stage {
        s if (s == 5 && difficulty == Difficulty::Easy) => println!("easy mode at stage 5"),
        s if difficulty == Difficulty::Normal => println!("Normal mode at stage {}", s), 
        s @ 10 | s @ 15 => println!("stage 10 or stage 15"),
        s => println!("stage = {}", s),
    }
}