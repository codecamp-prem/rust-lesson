enum Color {
    Red,
    Green,
    Blue,
    Dark
}

fn main(){
    color_name(Color::Blue);
}

fn color_name(passed_color: Color){
    match passed_color{
        Color::Blue=>println!("It's Blue"), 
        Color::Dark=>println!("It's Dark"), 
        Color::Green=>println!("It's Green"), 
        Color::Red=>println!("It's Red"), 
    }
}