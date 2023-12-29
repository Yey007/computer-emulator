use crate::connectable::device_pin::DevicePin;
use crate::connectable::device_port::DevicePort;
use crate::device::Device;

pub struct Console {
    ascii: DevicePort<8>,
    write: DevicePin,
    previous_write: bool
}

impl Console {
    pub fn new() -> Self {
        Console {
            ascii: DevicePort::new(),
            write: DevicePin::new(),
            previous_write: false
        }
    }
    
    pub fn ascii_port(&mut self) -> &mut DevicePort<8> {
        &mut self.ascii
    }
    
    pub fn write_pin(&mut self) -> &mut DevicePin {
        &mut self.write
    }
}

impl Device for Console {
    fn tick(&mut self) {
        let new = self.write.read() == 1u8.into();

        if new && !self.previous_write {
            let ascii: u8 = self.ascii.read().into();
            print!("{}", ascii as char)
        }

        self.previous_write = new
    }
}