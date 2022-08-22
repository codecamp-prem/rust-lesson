#[derive(Debug, Clone, Copy)]
enum Position{
    Manager,
    Supervisor,
    Worker
}

#[derive(Debug, Clone, Copy)]
struct Employee{
    position: Position,
    work_hours: i64,
}

fn print_employee(emp: Employee){
    println!("{:?}", emp);
}

fn main(){
    let jobless = Employee{
        position: Position::Manager,
        work_hours: 45,
    };
    print_employee(jobless);
    print_employee(jobless);
}
