use crate::bytes_to_store_bits;
use crate::connectable::Connectable;
use crate::un::U;

pub struct DevicePort<'a, const N: usize> where [(); bytes_to_store_bits!(N)]: Sized {
    value: U<N>,
    connection: Option<&'a dyn Connectable<'a, N>>
}

impl<'a, const N: usize> DevicePort<'a, N> where [(); bytes_to_store_bits!(N)]: Sized {
    pub fn new() -> Self<> {
        DevicePort {
            value: 0u8.into(),
            connection: None
        }
    }

    pub fn read(&self) -> U<N> {
        self.value
    }

    pub fn write(&mut self, value: U<N>) {
        self.value = value
    }
}

impl<'a, const N: usize> Connectable<'a, N> for DevicePort<'a, N> where [(); bytes_to_store_bits!(N)]: Sized {
    fn propagate(&self, value: U<N>) {
        todo!()
    }

    fn connect_to(&mut self, other: &'a dyn Connectable<'a, N>) {
        self.connection = Some(other);
    }
}
