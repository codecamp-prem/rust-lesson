trait Fall{
    fn hit_ground(&self);
}

struct Apple;
impl Fall for Apple{
    fn hit_ground(&self){
        println!("Falled on Newton's head");
    }
}

struct Cat;
impl Fall for Cat{
    fn hit_ground(&self){
        println!("Safely bounced!");
    }
}

// fn that ultilize the trait
// notice that fn fall is not same as impl Fall: letterCase matter
fn fall(thing: impl Fall){
    thing.hit_ground();
}

fn main(){
    fall(Apple{});
    fall(Cat{})
}