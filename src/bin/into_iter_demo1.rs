use std::collections::HashMap;

 #[derive(Debug, Hash, Eq, PartialEq)]
 enum Fruit{
    Apple,
    Banana,
    Orange,
 }

 struct FruitStand{
    fruit: HashMap<Fruit, u32>,
 }

 impl IntoIterator for FruitStand{
    type Item = (Fruit, u32);
    type IntoIter = std::collections::hash_map::IntoIter<Fruit, u32>;
    fn into_iter(self) -> Self::IntoIter{
        self.fruit.into_iter() // convention in Rust is into_iter() means moving values
    }
 }

 impl<'a> IntoIterator for &'a FruitStand{
    type Item = (&'a Fruit, &'a u32);
    type IntoIter = std::collections::hash_map::Iter<'a, Fruit, u32>;
    fn into_iter(self) -> Self::IntoIter{
        self.fruit.iter() // convention in Rust is iter() means borrowing values
    }
 }

 impl<'a> IntoIterator for &'a mut FruitStand{
    type Item = (&'a Fruit, &'a mut u32); // we don't want to change the keys so no mut infront of Fruit
    type IntoIter = std::collections::hash_map::IterMut<'a, Fruit, u32>;
    fn into_iter(self) -> Self::IntoIter{
        self.fruit.iter_mut() // convention in Rust is iter_mut() means changing the values with mut
    }
 }

 fn main(){
    let mut fruit = HashMap::new();
    fruit.insert(Fruit::Apple, 3);
    fruit.insert(Fruit::Banana, 5);
    fruit.insert(Fruit::Orange, 6);

    let fruit = fruit;
    
    //let store = FruitStand{fruit};

    // Move called IntoIter<Fruit, u32>, we cannot iter more than once
    //for (fruit, stock) in store.into_iter(){
    /*for (fruit, stock) in store{ //.into_iter() will be called implicitly
        println!("{:?}: {:?}", fruit, stock);
    }
    */

    // borrow called Iter<'a, Fruit, u32>;, we can iter called more than once
    /*for (fruit, stock) in &store{ 
        println!("{:?}: {:?}", fruit, stock);
    }
    for (fruit, stock) in &store{ 
        println!("{:?}: {:?}", fruit, stock);
    }*/

    // borrow with mutabilty called  IterMut<'a, Fruit, u32>;, we can iterate and change the value
    let mut store = FruitStand{fruit}; // we have to have mut value to start with
    for (fruit, stock) in &mut store{
        *stock += 10; //to change something we dereference with *, here we are adding 10 to all stock value. 
        println!("{:?}: {:?}", fruit, stock);
    }
 }