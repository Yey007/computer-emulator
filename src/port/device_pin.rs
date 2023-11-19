use crate::port::device_port::DevicePort;
use crate::port::Port;

pub struct DevicePin(DevicePort<1>);

impl DevicePin {
    pub fn read(&self) -> bool {
        self.0.read()[0]
    }
    
    pub fn write(&mut self, value: bool) {
        self.0.write([value])
    }
}