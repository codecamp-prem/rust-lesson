const MAX_G:f64 = 10.2;

fn top_gun_pilot(bearable_g: f64) -> f64{
    if bearable_g > MAX_G{
        MAX_G
    }else{
        bearable_g
    }
}

fn main(){
    let maverick = top_gun_pilot(10.3);
    println!("Maverick top g: {}", maverick); // Maverick top g: 10.2
}