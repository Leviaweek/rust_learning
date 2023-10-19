pub struct Coffee{
    pub water: usize,
    pub milk: usize,
    pub beans: usize,
    pub cups: usize,
    pub cost: usize
}
impl Coffee{
    pub fn new(water: usize, milk: usize, beans: usize, cups: usize, cost:usize) -> Coffee{
        Coffee {water, milk, beans, cups, cost};
    }
}