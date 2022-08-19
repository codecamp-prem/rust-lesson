struct Celcius{
    degree_c : i32,
}

impl Celcius{
    fn freezing() -> Self {
        Self {degree_c: 0}
    }
    fn boiling() -> Self{
        Self {degree_c: 100}
    }
    fn show_temp(&self){
        println!("{:?} degree in Celcius", self.degree_c);
    }
}

fn main(){
    let temp = Celcius{degree_c: 27};
    temp.show_temp();

    let freezing_point = Celcius::freezing(); // similiar to let freezing_point = Celcius{degree_c: 0};
    freezing_point.show_temp();
    freezing_point.show_temp(); // can be called multiple times

    let boiling_point = Celcius::boiling();
    boiling_point.show_temp();
}