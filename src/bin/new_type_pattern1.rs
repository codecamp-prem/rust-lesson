// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//

#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
#[derive(Debug)]
struct ShirtColor(Color);
impl ShirtColor{
    fn new(color: Color) -> Self{
        Self(color)
    }
}

#[derive(Debug)]
struct ShoesColor(Color);
impl ShoesColor{
    fn new(color: Color) -> Self{
        Self(color)
    }
}

#[derive(Debug)]
struct PantsColor(Color);
impl PantsColor{
    fn new(color: Color) -> Self{
        Self(color)
    }
}

// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing
fn print_shirt_color(color: ShirtColor){
    println!("Shirt Color: {:?}", color);
}
fn print_shoe_color(color: ShoesColor){
    println!("Shoe Color: {:?}", color);
}
fn print_pant_color(color: PantsColor){
    println!("Pants Color: {:?}", color);
}

fn main() {
    let shirt_color = ShirtColor::new(Color::Green);
    let shoe_color = ShoesColor::new(Color::Brown);
    let pant_color = PantsColor::new(Color::Gray);

    print_shirt_color(shirt_color);
    print_shoe_color(shoe_color);
    print_pant_color(pant_color);
}