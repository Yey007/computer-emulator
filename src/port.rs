pub mod device_port;
pub mod device_pin;

pub trait Port<const N: usize> {
    fn read(&self) -> [bool; N];
    fn write(&mut self, value: [bool; N]);
}
