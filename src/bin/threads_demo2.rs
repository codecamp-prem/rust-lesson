use std::thread;
use std::time::Duration;
use std::thread::JoinHandle;

fn main(){
    let value: JoinHandle<usize> = thread::spawn(move ||{
        thread::sleep(Duration::from_secs(1));
        42
    });
    println!("Waiting on thread");

    match value.join(){
        Ok(n) => println!("Thread returned value: {}", n),
        Err(e) => println!("error joining thread: {:?}", e),
    }
}