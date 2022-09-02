trait Perimeter{
    fn perimeter_calculation(&self)->i32;
}

struct Square{
    side: i32,
}
impl Perimeter for Square{
    fn perimeter_calculation(&self)->i32{
        self.side*4
    }
}

struct Triangle{
    a: i32,
    b: i32,
    c: i32,
}
impl Perimeter for Triangle{
    fn perimeter_calculation(&self)->i32{
        self.a + self.b + self.c
    }
}

fn print_perimeter(shape: impl Perimeter){
    let perimeter = shape.perimeter_calculation();
    println!("perimeter: {:?}", perimeter);
}

fn main(){
    print_perimeter(Square{side: 4});
    print_perimeter(Triangle{a: 7, b:6, c:3});
}