use std::rc::Rc;
use crate::bytes_to_store_bits;
use crate::connectable::Connectable;
use crate::notif_cell::NotifCell;
use crate::un::U;

pub struct DevicePort<const N: usize> where [(); bytes_to_store_bits!(N)]: Sized {
    value: Rc<NotifCell<U<N>>>,
}

impl<const N: usize> DevicePort<N> where [(); bytes_to_store_bits!(N)]: Sized {
    pub fn new() -> Self<> {
        DevicePort {
            value: 0u8.into(),
        }
    }

    pub fn read(&self) -> U<N> {
        self.value.get()
    }

    pub fn write(&mut self, value: U<N>) {
        self.value.set(value)
    }
}

impl<const N: usize> Connectable<N> for DevicePort<N> where [(); bytes_to_store_bits!(N)]: Sized {
    fn get_value_ref(&self) -> Rc<NotifCell<U<N>>> {
        self.value.clone()
    }

    fn connect_to(&mut self, other: &dyn Connectable<N>) {
        self.value = other.get_value_ref();
    }
}
