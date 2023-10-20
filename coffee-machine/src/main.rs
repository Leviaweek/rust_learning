mod components;

use std::io::BufRead;
use components::*;

fn main() -> std::io::Result<()>{
    let mut machine = CoffeeMachine::new();
    let mut stdin = std::io::stdin().lock();
    loop {
        machine.print_interface()?;
        if machine.state == MachineState::Exit {
            break Ok(());
        }
        let mut input = String::new();
        stdin.read_line(&mut input)?;
        match machine.input_handler(&input) {
            Ok(_) => {}
            Err(err) => eprintln!("{}", err),
        }
    }
}
