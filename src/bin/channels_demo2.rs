use crossbeam_channel::unbounded;
use std::thread;

enum WorkerMsg{
    PrintData(String),
    Sum(i64, i64),
    Quit,
}

enum MainMsg{
    SumResult(i64),
    WorkerQuit,
}

fn main(){
    let (worker_tx, worker_rx) = unbounded(); // communicating with only workers thread
    let (main_tx, main_rx) = unbounded(); // communicating with only main thread

    let worker = thread::spawn(move || loop{
        match worker_rx.recv(){
            Ok(msg) => match msg{
                WorkerMsg::PrintData(d)=>println!("Worker: {}", d),
                WorkerMsg::Sum(lhs, rhs)=>{
                    println!("Worker thread summing the result....!");
                    main_tx.send(MainMsg::SumResult(lhs + rhs));
                    ()
                },//println!("{}+{}={}", lhs, rhs, (lhs+rhs)),
                WorkerMsg::Quit => {
                    println!("Worker thread terminating!");
                    main_tx.send(MainMsg::WorkerQuit);
                    break;
                }
            },
            Err(e)=>{
                println!("Worker Disconnected!!");
                main_tx.try_send(MainMsg::WorkerQuit);
                break;
            },
        }
    });
    worker_tx.send(WorkerMsg::PrintData("hello from main".to_owned()));
    worker_tx.send(WorkerMsg::Sum(67, 76));
    worker_tx.send(WorkerMsg::Quit); //thread is terminated but channels still the data to be read.
    //drop(worker_tx); //drop will diconnect the sender of the channel and will invoke Err condition.

    while let Ok(msg) = main_rx.recv(){ 
        match msg {
            MainMsg::SumResult(answer) => println!("Main: answer = {}", answer),
            MainMsg::WorkerQuit => println!("Main: worker terminated!"),
        }
    }
    worker.join();
} 