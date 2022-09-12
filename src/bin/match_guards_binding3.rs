struct Vehicle{
    km: usize,
    year: usize,
}

fn main(){
    let car = Vehicle{
        km: 100_000,
        year: 2019,
    };

    // match struct with if
    match car{
        Vehicle{ km, year } if km == 0 && year == 2019 => println!("Brand New 2019 Vehicle"),
        Vehicle{ km, .. } if km <= 50_000 => println!("Under 50k km"),
        Vehicle{ km, .. } if km >= 80_000  && km <= 99_000 => println!("At least 80k km and below 90k km"),
        Vehicle{ year, .. } if year == 2019 => println!("Made in year 2019"),
        Vehicle{ .. } => println!("Other Mileage!!"),
    }
}