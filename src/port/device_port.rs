use crate::port::Port;

pub struct DevicePort<const N: usize> {
    value: [bool; N],
}

impl<const N: usize> Port<N> for DevicePort<N> {
    fn read(&self) -> [bool; N] {
        self.value 
    }

    fn write(&mut self, value: [bool; N]) {
        self.value = value
    }
}
