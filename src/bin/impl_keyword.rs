struct Temperature{
    degree_f: f64,
}

impl Temperature{
    fn show_temp(temp: Temperature){
        println!("{:?} degree in F", temp.degree_f);
    }
}

fn main(){
    let weather = Temperature{
        degree_f: 102.08,
    };
    Temperature::show_temp(weather);
}