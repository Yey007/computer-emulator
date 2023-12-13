use crate::bytes_to_store_bits;
use crate::un::U;

pub mod device_port;
pub mod device_pin;
mod wire;

pub trait Port<const N: usize> where [(); bytes_to_store_bits!(N)]: Sized {
    fn read(&self) -> U<N>;
    fn write(&mut self, value: U<N>);
}
