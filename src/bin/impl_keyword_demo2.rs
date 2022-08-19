enum Color{
    Red,
    Green,
    Blue,
}

impl Color{
    fn print(&self){
        match self{
            Color::Blue => println!("Blue"),
            Color::Red => println!("Red"),
            Color::Green => println!("Green"),
        }
    }
}

struct Dimensions{
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions{
    fn print(&self){
        println!("Width: {:?}", self.width);
        println!("Height: {:?}", self.height);
        println!("Depth: {:?}", self.depth);
    }
}

struct Box{
    dimensions: Dimensions,
    weight: f64,
    color: Color,
}

impl Box{
    fn create_box(dimensions: Dimensions, weight: f64, color: Color) -> Self{
        Self {
            dimensions,
            weight,
            color,
        }
    }
    fn print_box(&self){
        println!("Weight: {:?}", self.weight);
        self.dimensions.print();
        self.color.print();       
    }
}

fn main(){
    let box_dimensions = Dimensions{
        width: 4.5,
        height: 5.4,
        depth: 4.0,
    };

    let make_box = Box::create_box(box_dimensions, 0.250, Color::Blue);
    make_box.print_box();
}