enum Flavor{
    Sweet,
    Lemon,
    Masala
}

struct Drink{
    flavor: Flavor,
    fluid_oz: f64,
}

fn print_drink(drink:Drink){
    match drink.flavor {
        Flavor::Sweet =>println!("flavor: Sweet"),
        Flavor::Lemon =>println!("flavor: Lemon"),
        Flavor::Masala =>println!("flavor: Masala"),
    }
    println!("oz: {:?}", drink.fluid_oz);
}

fn main(){
    let sweet = Drink{
        flavor: Flavor::Sweet,
        fluid_oz: 7.0
    };
    print_drink(sweet);

    let masala = Drink{
        flavor: Flavor::Masala,
        fluid_oz: 9.0
    };
    print_drink(masala);
}