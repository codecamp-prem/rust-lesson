use std::thread;

fn main(){
    let data = vec!['a', 'b', 'c'];
    let capitailize = thread::spawn(move || {
        let data: Vec<char> = data.iter().map(|c| c.to_ascii_uppercase()).collect();
        data
    });
    println!("Waiting for value ...");
    match capitailize.join(){
        Ok(c) => println!("Value: {:?}", c),
        Err(e) => println!("Error: {:?}", e)
    }
}