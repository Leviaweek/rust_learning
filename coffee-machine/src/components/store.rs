use std::fmt::{Display, Formatter};
use super::Coffee;
use strum_macros::Display;

pub struct Store {
    pub water: usize,
    pub milk: usize,
    pub beans: usize,
    pub cups: usize,
    pub money: usize,
}

impl Default for Store {
    fn default() -> Self {
        Store {
            water: 400,
            milk: 540,
            beans: 120,
            cups: 9,
            money: 550,
        }
    }
}

impl Display for Store{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "I have:")?;
        writeln!(f, "{} ml of water", self.water)?;
        writeln!(f, "{} ml of milk", self.milk)?;
        writeln!(f, "{} mg of beans", self.beans)?;
        writeln!(f, "{} cups", self.cups)?;
        write!(f, "{}$ money", self.money)
    }
}

const MAX_WATER: usize = 10000;
const MAX_MILK: usize = 10000;
const MAX_BEANS: usize = 1000;
const MAX_CUPS: usize = 100;

impl Store{
    pub fn fill_water(&mut self, count: usize) -> Result<(), StoreError>{
        if self.water + count > MAX_WATER{
            return Err(StoreError::IncorrectFill(Ingredient::Water));
        }
        self.water += count;
        Ok(())
    }
    pub fn fill_milk(&mut self, count: usize) -> Result<(), StoreError>{
        if self.milk + count > MAX_MILK{
            return Err(StoreError::IncorrectFill(Ingredient::Milk));
        }
        self.milk += count;
        Ok(())
    }
    pub fn fill_beans(&mut self, count: usize) -> Result<(), StoreError>{
        if self.beans + count > MAX_BEANS{
            return Err(StoreError::IncorrectFill(Ingredient::Beans));
        }
        self.beans += count;
        Ok(())
    }
    pub fn fill_cups(&mut self, count: usize) -> Result<(), StoreError>{
        if self.cups + count > MAX_CUPS{
            return Err(StoreError::IncorrectFill(Ingredient::Cups));
        }
        self.cups += count;
        Ok(())
    }
    pub fn take_money(&mut self) -> usize{
        let money = self.money;
        self.money = 0;
        money
    }
    pub fn process_purchase(&mut self, coffee: &Coffee) -> Result<(), StoreError>{
        let mut ingredients: Vec<Ingredient> = Vec::with_capacity(4);
        if self.water < coffee.water {
            ingredients.push(Ingredient::Water);
        }
        if self.milk < coffee.milk {
            ingredients.push(Ingredient::Milk);
        }
        if self.beans < coffee.beans {
            ingredients.push(Ingredient::Beans);
        }
        if self.cups < coffee.cups {
            ingredients.push(Ingredient::Cups);
        }
        if !ingredients.is_empty(){
            return Err(StoreError::NotEnoughIngredient(ingredients));
        }
        self.water -= coffee.water;
        self.milk -= coffee.milk;
        self.beans -= coffee.beans;
        self.cups -= coffee.cups;
        self.money += coffee.cost;
        Ok(())
    }
}

#[derive(Debug)]
pub enum StoreError{
    NotEnoughIngredient(Vec<Ingredient>),
    IncorrectFill(Ingredient),
}

impl Display for StoreError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StoreError::NotEnoughIngredient(ingredients) =>{ 
                let text = &ingredients.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ");
                write!(f, "Not enough {}", text)?;
            }
            StoreError::IncorrectFill(ingredient) => write!(f, "Too much {}", ingredient)?
        }
        Ok(())
    }
}

#[derive(Debug, Display)]
pub enum Ingredient{
    #[strum(serialize = "water")]
    Water,
    #[strum(serialize = "milk")]
    Milk,
    #[strum(serialize = "beans")]
    Beans,
    #[strum(serialize = "cups")]
    Cups,
    #[allow(dead_code)]
    #[strum(serialize = "money")]
    Money
}

impl std::error::Error for StoreError{}