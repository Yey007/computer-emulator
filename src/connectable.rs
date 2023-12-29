use std::cell::RefCell;
use std::rc::Rc;
use crate::bytes_to_store_bits;
use crate::store::Store;
use crate::un::U;

pub mod device_port;
pub mod device_pin;
pub mod spliter;

pub trait Connectable<const N: usize> where [(); bytes_to_store_bits!(N)]: Sized {
    fn get_value_ref(&self) -> Rc<RefCell<Store<U<N>>>>;
    fn connect_to(&mut self, other: &dyn Connectable<N>);
}
