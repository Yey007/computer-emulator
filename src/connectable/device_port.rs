use std::cell::RefCell;
use std::rc::Rc;
use crate::bytes_to_store_bits;
use crate::connectable::Connectable;
use crate::un::U;

pub struct DevicePort<const N: usize> where [(); bytes_to_store_bits!(N)]: Sized {
    value: U<N>,
    connection: Option<Rc<RefCell<dyn Connectable<N>>>>,
}

impl<const N: usize> DevicePort<N> where [(); bytes_to_store_bits!(N)]: Sized {
    pub fn new() -> Self <> {
        DevicePort {
            value: 0u8.into(),
            connection: None,
        }
    }

    pub fn read(&mut self) -> U<N> {
        if let Some(connection) = &self.connection {
            self.value = connection.borrow().pull_value()
        }
        self.value
    }

    pub fn write(&mut self, value: U<N>) {
        self.value = value;
    }
}

impl<const N: usize> Connectable<N> for DevicePort<N> where [(); bytes_to_store_bits!(N)]: Sized {
    fn pull_value(&self) -> U<N> {
        self.value
    }

    fn connect_to(&mut self, other: Rc<RefCell<dyn Connectable<N>>>) {
        self.connection = Some(other)
    }
}
