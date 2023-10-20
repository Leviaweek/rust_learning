pub struct Coffee{
    pub name: String,
    pub water: usize,
    pub milk: usize,
    pub beans: usize,
    pub cups: usize,
    pub cost: usize
}
impl Coffee{
    pub fn new(name: String, water: usize, milk: usize, beans: usize, cups: usize, cost:usize) -> Coffee{
        Coffee {name, water, milk, beans, cups, cost}
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_coffee_new() {
        let coffee = Coffee::new("Test".to_owned(), 1, 2, 3, 4, 5);
        assert_eq!(coffee.name, "Test");
        assert_eq!(coffee.water, 1);
        assert_eq!(coffee.milk, 2);
        assert_eq!(coffee.beans, 3);
        assert_eq!(coffee.cups, 4);
        assert_eq!(coffee.cost, 5);
    }
}