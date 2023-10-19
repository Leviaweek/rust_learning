mod coffee_machine;

use coffee_machine::CoffeeMachine;

fn main() -> std::io::Result<()>{
    let mut machine = CoffeeMachine::new();
    let stdin = std::io::stdin();
    loop {
        machine.print_interface()?;
        if machine.state == coffee_machine::MachineState::Exit {
            break Ok(());
        }
        let mut input = String::new();
        stdin.read_line(&mut input)?;
        match machine.input_handler(&input) {
            Ok(_) => {}
            Err(err) => println!("{}", err),
        }
    }
}
