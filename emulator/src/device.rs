pub mod console;
pub mod connectable;
mod store;

pub trait Device {
    fn tick(&mut self, tick: u32);
}
