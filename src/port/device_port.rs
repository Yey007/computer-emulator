use crate::bytes_to_store_bits;
use crate::port::Port;
use crate::un::U;

pub struct DevicePort<const N: usize> where [(); bytes_to_store_bits!(N)]: Sized {
    value: U<N>,
}

impl<const N: usize> DevicePort<N> where [(); bytes_to_store_bits!(N)]: Sized {
    pub fn new() -> Self<> {
        DevicePort {
            value: 0u8.into()
        }
    }
}

impl<const N: usize> Port<N> for DevicePort<N> where [(); bytes_to_store_bits!(N)]: Sized {
    fn read(&self) -> U<N> {
        self.value 
    }

    fn write(&mut self, value: U<N>) {
        self.value = value
    }
}
