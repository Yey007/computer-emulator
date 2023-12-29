use std::cell::RefCell;
use std::rc::Rc;
use crate::bytes_to_store_bits;
use crate::connectable::Connectable;
use crate::store::Store;
use crate::un::U;

pub struct DevicePort<const N: usize> where [(); bytes_to_store_bits!(N)]: Sized {
    store: Rc<RefCell<Store<U<N>>>>,
}

impl<const N: usize> DevicePort<N> where [(); bytes_to_store_bits!(N)]: Sized {
    pub fn new() -> Self<> {
        DevicePort {
            store: Rc::new(RefCell::new(Store::new(0u8.into()))),
        }
    }

    pub fn read(&self) -> U<N> {
        self.store.borrow().get()
    }

    pub fn write(&mut self, value: U<N>, tick: u32) {
        self.store.borrow_mut().set(value, tick);
    }
}

impl<const N: usize> Connectable<N> for DevicePort<N> where [(); bytes_to_store_bits!(N)]: Sized {
    fn get_value_ref(&self) -> Rc<RefCell<Store<U<N>>>> {
        self.store.clone()
    }

    fn connect_to(&mut self, other: &dyn Connectable<N>) {
        self.store = other.get_value_ref();
    }
}
