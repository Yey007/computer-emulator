use crate::connectable::device_pin::DevicePin;
use crate::connectable::device_port::DevicePort;
use crate::device::Device;

pub struct Console<'a> {
    ascii: DevicePort<'a, 8>,
    write: DevicePin<'a>,
    previous_write: bool
}

impl<'a> Console<'a> {
    pub fn new() -> Self {
        Console {
            ascii: DevicePort::new(),
            write: DevicePin::new(),
            previous_write: false
        }
    }
    
    pub fn ascii_port(&mut self) -> &mut DevicePort<'a, 8> {
        &mut self.ascii
    }
    
    pub fn write_pin(&mut self) -> &mut DevicePin<'a> {
        &mut self.write
    }
}

impl<'a> Device for Console<'a> {
    fn tick(&mut self) {
        let new = self.write.read() == 1u8.into();

        if new && !self.previous_write {
            let ascii: u8 = self.ascii.read().into();
            print!("{}", ascii as char)
        }

        self.previous_write = new
    }
}