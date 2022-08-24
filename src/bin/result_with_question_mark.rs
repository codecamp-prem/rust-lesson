enum Position{
    Maintenance,
    Marketing,
    Manager,
    LineSupervisor,
    KitchenStaff,
    AssemblyTech,
}

enum Status{
    Active,
    Terminated,
}

struct Employee{
    position: Position,
    status: Status,
}

fn have_access(employee: &Employee)->Result<(), String>{
    match employee.status{
        Status::Terminated => return Err("Terminated".to_owned()),
        _ => (),
    }

    match employee.position{
        Position::Maintenance => Ok(()),
        Position::Marketing => Ok(()),
        Position::Manager => Ok(()),
        _ => Err("Invalid Position".to_owned()),
    }
}

fn print_access(employee: &Employee)->Result<(), String>{
    let can_access = have_access(employee)?;
    println!("Access OK");
    Ok(())
}

fn main(){
    let manager = Employee{
        position: Position::Manager,
        status: Status::Active,
    };

    match print_access(&manager){
        Err(e) => println!("Access Denied: {:?}", e),
        _ => (),
    }

    let low_level = Employee{
        position: Position::KitchenStaff,
        status: Status::Active,
    };

    match print_access(&low_level){
        Err(e) => println!("Access Denied: {:?}", e),
        _ => (),
    }
}