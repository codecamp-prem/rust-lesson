struct Faternity{
    name: String,
    locker: Option<i32>,
}

fn main(){
    let student = Faternity{
        name: "Jackson".to_owned(),
        locker: None,
    };
    println!("Student's name: {:?}", student.name);
    match student.locker{
        Some(got_locker) => println!("Locker no.: {:?}", got_locker),
        None => println!("Student don't have locker."),
    }
}