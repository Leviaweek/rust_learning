use core::fmt;
use std::io::{ErrorKind, Error, Write};

pub struct CoffeeMachine{
    pub state: MachineState,
    coffees: Vec<Coffee>,
    water: usize,
    milk: usize,
    beans: usize,
    cups: usize,
    money: usize
}

impl CoffeeMachine{
    pub fn new() -> CoffeeMachine{
        let mut coffees = Vec::<Coffee>::with_capacity(3);
        coffees.push(Coffee::new("Espresso".to_owned(), 250, 0, 16, 1, 4));
        coffees.push(Coffee::new("Latte".to_owned(), 350, 75, 20, 1, 7));
        coffees.push(Coffee::new("Cappucino".to_owned(), 200, 100, 12, 1, 6));
        CoffeeMachine { state: MachineState::MainMenu, coffees, water: 400, milk: 540, beans: 120, cups: 9, money: 550 }
    }
    pub fn print_interface(&self) -> std::io::Result<()>{
        let mut stdout = std::io::stdout().lock();
        match self.state{
            MachineState::MainMenu => print!("Write action (buy, fill, take, remaining, exit): "),
            MachineState::BuyMenu => {
                let mut text = "Choose coffee: ".to_owned();
                for i in 0..self.coffees.len(){
                    text += &format!("{} - {} ", i + 1, self.coffees[i].name);
                }
                print!("{}: ", text);
            },
            MachineState::FillWater => print!("Input water in mililiters (10000 max): "),
            MachineState::FillMilk => print!("Input milk in mililiters (10000 max): "),
            MachineState::FillBeans => print!("Input beans in miligrams (1000 max): "),
            MachineState::FillCups => print!("Input cups count (100 max): "),
            MachineState::Exit => println!("Goodbye!")
        }
        stdout.flush()?;
        Ok(())
    }
    pub fn input_handler(&mut self, input: &str) -> std::io::Result<()>{
        let processed_input = input.trim().to_ascii_lowercase();
        match self.state{
            MachineState::MainMenu => {
                match processed_input.as_str(){
                    "buy" => self.state = MachineState::BuyMenu,
                    "fill" => self.state = MachineState::FillWater,
                    "take" => self.take(),
                    "remaining" => self.remaining(),
                    "exit" => self.state = MachineState::Exit,
                    _ => return Err(Error::new(ErrorKind::InvalidInput, "Incorrect input"))
                }
                return Ok(());
            }
            MachineState::BuyMenu => {
                match processed_input.as_str(){
                    "back" => self.state = MachineState::MainMenu,
                    _ => {
                        let result = processed_input.parse::<usize>();
                        let number = match result{
                            Ok(val) => val,
                            Err(_) => return Err(Error::new(ErrorKind::InvalidInput, "Input number or back"))
                        };
                            match self.make_coffee(number - 1){
                                Ok(_) => println!("I make you a coffee!"),
                                Err(err) => println!("{}", err)
                            }
                            self.state = MachineState::MainMenu;
                    }
                }
            }
            MachineState::FillWater |
            MachineState::FillMilk |
            MachineState::FillBeans |
            MachineState::FillCups =>{
                let number = match processed_input.parse::<usize>(){
                    Ok(val) => val,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidInput, "Input only number"))
                };
                match self.fill(number){
                    Ok(_) => {},
                    Err(err) => println!("{}", err)
                }
            }
            MachineState::Exit => {
                println!("Goodbye");
            }
        }
        Ok(())
    }
    fn take(&mut self){
        println!("I take you {}$", self.money);
        self.money = 0;
    }
    fn remaining(&self){
        println!("I have:
        {} ml of water
        {} ml of milk
        {} mg of beans
        {} cups
        {}$ money", self.water, self.milk, self.beans, self.cups, self.money);
    }
    fn fill(&mut self, count: usize) -> Result<(), MachineErrors> {
        if count == 0{
            return Err(MachineErrors::IncorrectFill("Error input".to_owned()));
        }
        match self.state{
            MachineState::FillWater => {
                if count + self.water > 10000{
                    return Err(MachineErrors::IncorrectFill("Too much water".to_owned()));
                }
                self.water += count;
                self.state = MachineState::FillMilk;
            }
            MachineState::FillMilk => {
                if count + self.milk > 10000{
                    return Err(MachineErrors::IncorrectFill("Too much milk".to_owned()));
                }
                self.milk += count;
                self.state = MachineState::FillBeans;
            }
            MachineState::FillBeans => {
                if count + self.beans > 1000{
                    return Err(MachineErrors::IncorrectFill("Too much beans".to_owned()));
                }
                self.beans += count;
                self.state = MachineState::FillCups;
            }
            MachineState::FillCups => {
                if count + self.cups > 100{
                    return Err(MachineErrors::IncorrectFill("Too much cups".to_owned()));
                }
                self.cups += count;
                self.state = MachineState::MainMenu;
            }
            _ => return Err(MachineErrors::IncorrectFill("Incorrect state".to_owned()))
        }
        Ok(())
    }
    fn make_coffee(&mut self, coffee_index: usize) -> Result<(), MachineErrors>{
        if coffee_index >= self.coffees.len(){
            return Err(MachineErrors::IncorrectFill("Incorrect coffee index".to_owned()));
        }
        let coffee = &self.coffees[coffee_index];
        if self.water < coffee.water{
            return Err(MachineErrors::NotEnoughtIngredient("Not enought water".to_owned()));
        }
        if self.milk < coffee.milk{
            return Err(MachineErrors::NotEnoughtIngredient("Not enought milk".to_owned()));
        }
        if self.beans < coffee.beans{
            return Err(MachineErrors::NotEnoughtIngredient("Not enought beans".to_owned()));
        }
        if self.cups < coffee.cups{
            return Err(MachineErrors::NotEnoughtIngredient("Not enought cups".to_owned()));
        }
        self.water -= coffee.water;
        self.milk -= coffee.milk;
        self.beans -= coffee.beans;
        self.cups -= coffee.cups;
        self.money += coffee.cost;
        Ok(())
    }
}

pub enum MachineState{
    MainMenu,
    BuyMenu,
    FillWater,
    FillMilk,
    FillBeans,
    FillCups,
    Exit
}

impl PartialEq for MachineState{
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

pub enum MachineErrors{
    NotEnoughtIngredient(String),
    IncorrectFill(String)
}

impl std::error::Error for MachineErrors{}

impl fmt::Display for MachineErrors{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            MachineErrors::NotEnoughtIngredient(err) => write!(f, "Ingredient error: {}", err),
            MachineErrors::IncorrectFill(err) => write!(f, "Fill error: {}", err)
        }
    }
}
impl std::fmt::Debug for MachineErrors{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotEnoughtIngredient(arg0) => f.debug_tuple("NotEnoughtIngredient").field(arg0).finish(),
            Self::IncorrectFill(arg0) => f.debug_tuple("IncorrectFill").field(arg0).finish(),
        }
    }
}

struct Coffee{
    name: String,
    water: usize,
    milk: usize,
    beans: usize,
    cups: usize,
    cost: usize
}
impl Coffee{
    pub fn new(name: String, water: usize, milk: usize, beans: usize, cups: usize, cost:usize) -> Coffee{
        Coffee {name, water, milk, beans, cups, cost}
    }
}