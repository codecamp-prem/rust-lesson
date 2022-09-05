// Topic: Generics & Structures

// Notes:
// * Examples of car bodies can be Truck, Car, Scooter
// * Examples of colors could be red, white, black
// * It is not necessary to have data fields or function implementations
//   for the vehicle bodies/colors

trait Body {} //marker trait since body of its is empty.
trait Color {}
// * Create a Vehicle structure that is generic over traits Body and Color
#[derive(Debug)]
struct Vehicle<B: Body, C: Color>{
    body: B,
    color: C,
}
// * Implement a 'new' function for Vehicle that allows it to have any body and any color
impl<B:Body, C:Color> Vehicle<B,C>{
    pub fn new(body: B, color: C)-> Self {
        Self{body, color}
    }
}

// * Create structures for vehicle bodies and vehicle colors and implement the
//   Body and Color traits for these structures
#[derive(Debug)]
struct Car;
impl Body for Car{}
#[derive(Debug)]
struct Bus;
impl Body for Bus{}

#[derive(Debug)]
struct Red;
impl Color for Red{}
#[derive(Debug)]
struct Blue;
impl Color for Blue{}

// * Create at least two different vehicles in the main function and print their info
fn main() {
    let red_bus = Vehicle::new(Bus, Red);
    let blue_car = Vehicle::new(Car, Blue);
    println!("{:?}", red_bus); // Vehicle { body: Bus, color: Red }
    println!("{:?}", blue_car); //Vehicle { body: Car, color: Blue }
}