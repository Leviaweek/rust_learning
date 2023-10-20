use std::io::{Error, ErrorKind, Write};
use super::*;


pub struct CoffeeMachine {
    pub state: MachineState,
    coffees: Vec<Coffee>,
    store:Store
}

impl CoffeeMachine {
    pub fn new() -> CoffeeMachine {
        let mut coffees: Vec<Coffee> = Vec::<Coffee>::with_capacity(3);
        coffees.push(Coffee::new("Espresso".to_owned(), 250, 0, 16, 1, 4));
        coffees.push(Coffee::new("Latte".to_owned(), 350, 75, 20, 1, 7));
        coffees.push(Coffee::new("Cappuccino".to_owned(), 200, 100, 12, 1, 6));
        CoffeeMachine {
            state: MachineState::MainMenu,
            coffees,
            store: Store::default()
        }
    }
    pub fn print_interface(&self) -> std::io::Result<()> {
        let mut stdout = std::io::stdout().lock();
        match self.state {
            MachineState::MainMenu => print!("Write action (buy, fill, take, remaining, exit): "),
            MachineState::BuyMenu => {
                let mut text = "Choose coffee: ".to_owned();
                for i in 0..self.coffees.len() {
                    text += &format!("{} - {} ", i + 1, self.coffees[i].name);
                }
                print!("{}: ", text);
            }
            MachineState::FillWater => print!("Input water in millilitres (10000 max): "),
            MachineState::FillMilk => print!("Input milk in millilitres (10000 max): "),
            MachineState::FillBeans => print!("Input beans in milligrams (1000 max): "),
            MachineState::FillCups => print!("Input cups count (100 max): "),
            MachineState::Exit => println!("Goodbye!")
        }
        stdout.flush()?;
        Ok(())
    }
    pub fn input_handler(&mut self, input: &str) -> std::io::Result<()> {
        let processed_input = input.trim().to_ascii_lowercase();
        match self.state {
            MachineState::MainMenu => {
                match processed_input.as_str() {
                    "buy" => self.state = MachineState::BuyMenu,
                    "fill" => self.state = MachineState::FillWater,
                    "take" => self.take(),
                    "remaining" => self.remaining(),
                    "exit" => self.state = MachineState::Exit,
                    _ => return Err(Error::new(ErrorKind::InvalidInput, "Incorrect input")),
                }
                return Ok(());
            }
            MachineState::BuyMenu => match processed_input.as_str() {
                "back" => self.state = MachineState::MainMenu,
                _ => {
                    let result = processed_input.parse::<usize>();
                    let number = match result {
                        Ok(val) => val,
                        Err(_) => {
                            return Err(Error::new(ErrorKind::InvalidInput, "Input correct number or back"))
                        }
                    };
                    if number == 0 || number > self.coffees.len() {
                        return Err(Error::new(ErrorKind::InvalidInput, "Input correct number or back"));
                    }
                    match self.make_coffee(number - 1) {
                        Ok(_) => println!("I make you a coffee!"),
                        Err(err) => eprintln!("{}", err),
                    }
                    self.state = MachineState::MainMenu;
                }
            },
            MachineState::FillWater
            | MachineState::FillMilk
            | MachineState::FillBeans
            | MachineState::FillCups => {
                let number = match processed_input.parse::<usize>() {
                    Ok(val) => val,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidInput, "Input only number")),
                };
                match self.fill(number) {
                    Ok(_) => {}
                    Err(err) => println!("{}", err),
                }
            }
            MachineState::Exit => {
                println!("Goodbye");
            }
        }
        Ok(())
    }
    fn take(&mut self) {
        println!("I give you {}$", self.store.take_money());
    }

    fn remaining(&self) {
        println!("{}", self.store)
    }
    fn fill(&mut self, count: usize) -> Result<(), Error> {
        let result = match self.state {
            MachineState::FillWater => {
                self.store.fill_water(count)
            }
            MachineState::FillMilk => {
                self.store.fill_milk(count)
            }
            MachineState::FillBeans => {
                self.store.fill_beans(count)
            }
            MachineState::FillCups => {
                self.store.fill_cups(count)
            }
            _ => return Err(Error::new(ErrorKind::InvalidInput, "Incorrect input"))
        };
        match result {
            Ok(_) => {
                match self.state {
                    MachineState::FillWater => self.state = MachineState::FillMilk,
                    MachineState::FillMilk => self.state = MachineState::FillBeans,
                    MachineState::FillBeans => self.state = MachineState::FillCups,
                    MachineState::FillCups => self.state = MachineState::MainMenu,
                    _ => {}
                }
            }
            Err(err) => eprintln!("{}", err),
        }
        Ok(())
    }
    fn make_coffee(&mut self, coffee_index: usize) -> Result<(), StoreError> {
        let coffee = &self.coffees[coffee_index];
        self.store.process_purchase(&coffee)?;
        Ok(())
    }
}

pub enum MachineState {
    MainMenu,
    BuyMenu,
    FillWater,
    FillMilk,
    FillBeans,
    FillCups,
    Exit,
}

impl PartialEq for MachineState {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}
