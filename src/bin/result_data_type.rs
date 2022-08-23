#[derive(Debug)]
enum MenuChoice{
    MainMenu,
    Start,
    Quit,
}

fn get_choice(input: &str) -> Result<MenuChoice, String>{
    match input{
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("Invalid choice found".to_owned()),
    }
}

fn print_choice(choice: &MenuChoice){
    println!("choice: {:?}", choice);
}

fn pick_choice(input: &str) -> Result<(), String>{
    let choice: MenuChoice = get_choice(input)?; // ? will automatically perform match operation, if result is Ok variant inner data will place into the choice.Err will get automatically return as Result Error as String
    print_choice(&choice);
    Ok(())
}
fn main(){
    let choice = pick_choice("start");
    println!("choice: {:?}", choice);
}