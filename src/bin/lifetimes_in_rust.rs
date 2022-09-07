#[derive(Debug)]
struct Cards {
    inner: Vec<IdCard>
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum City{
    Barland,
    Bazopolis,
    Fooville,
}

#[derive(Debug)]
struct IdCard{
    name: String,
    age: u8,
    city: City,
}

impl IdCard{
    pub fn new(name: &str, age: u8, city: City) -> Self{
        Self{
            name: name.to_string(),
            age,
            city,
        }
    }
}

fn new_ids() -> Cards{
    Cards {
        inner: vec![
            IdCard::new("Jane", 1, City::Fooville),
            IdCard::new("John", 10, City::Barland),
            IdCard::new("Tom", 20, City::Barland),
            IdCard::new("Dick", 30, City::Bazopolis),
            IdCard::new("Harry", 40, City::Bazopolis),
        ],
    }
}

#[derive(Debug)]
struct YoungPeople<'a>{
    inner: Vec<&'a IdCard>
}

impl<'a> YoungPeople<'a>{
    fn living_in_fooville(&self) -> Self{
        Self{
            inner: self.inner.iter()
            .filter(|id| id.city == City::Fooville)
            .map(|id| *id).collect(), // mapping with * will cause the borrow to disappear
        }
    }
}

fn main(){
    let ids = new_ids();
    let young_gen = YoungPeople{
        inner: ids.inner.iter().filter(|id| id.age <= 20).collect(),
    };
    println!("ids");
    for id in ids.inner.iter(){
        println!("{:?}", id);
    }
    println!("\nYoung People");
    for id in young_gen.inner.iter(){
        println!("{:?}", id);
    }

    println!("\nYoung People Living in foovile");
    for id in young_gen.living_in_fooville().inner.iter(){
        println!("{:?}", id);
    }
}