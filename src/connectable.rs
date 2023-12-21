use crate::bytes_to_store_bits;
use crate::un::U;

pub mod device_port;
pub mod device_pin;
pub mod spliter;

pub trait Connectable<'a, const N: usize> where [(); bytes_to_store_bits!(N)]: Sized {
    fn propagate(&self, value: U<N>);
    fn connect_to(&mut self, other: &'a dyn Connectable<'a, N>);
}
