pub mod console;

pub trait Device {
    fn tick(&mut self, tick: u32);
}