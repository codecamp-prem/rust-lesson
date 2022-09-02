trait CheckIn{
    fn check_in(&self);
    fn process(&self);
}

struct Passenger;
impl CheckIn for Passenger{
    fn check_in(&self){
        println!("Passenger checking in.");
    }
    fn process(&self){
        println!("Passenger in the seat.");
    }
}

struct Pilot;
impl CheckIn for Pilot{
    fn check_in(&self){
        println!("Pilot checking in.");
    }
    fn process(&self){
        println!("Pilot in the cockpit.");
    }
}

struct Cargo;
impl CheckIn for Cargo{
    fn check_in(&self){
        println!("Cargo in trolley.");
    }
    fn process(&self){
        println!("Cargo in the Storage.");
    }
}

fn status<T: CheckIn>(item: T){
    item.check_in();
    item.process();
}

fn main(){
    let prem = Passenger;
    let akram = Pilot;
    let trekking_bag = Cargo;
    let suitcase = Cargo;
    status(prem);
    status(akram);
    status(trekking_bag);
    status(suitcase);
}