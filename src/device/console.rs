use std::cell::RefCell;
use std::rc::Rc;
use crate::connectable::device_pin::DevicePin;
use crate::connectable::device_port::DevicePort;
use crate::device::Device;

pub struct Console {
    ascii: Rc<RefCell<DevicePort<8>>>,
    write: Rc<RefCell<DevicePin>>,
    previous_write: bool
}

impl Console {
    pub fn new() -> Self {
        Console {
            ascii: Rc::new(RefCell::new(DevicePort::new())),
            write: Rc::new(RefCell::new(DevicePin::new())),
            previous_write: false
        }
    }
    
    pub fn ascii_port(&mut self) -> Rc<RefCell<DevicePort<8>>> {
        self.ascii.clone()
    }
    
    pub fn write_pin(&mut self) -> Rc<RefCell<DevicePin>> {
        self.write.clone()
    }
}

impl Device for Console {
    fn tick(&mut self) {
        let new = self.write.borrow_mut().read() == 1u8.into();

        if new && !self.previous_write {
            let ascii: u8 = self.ascii.borrow_mut().read().into();
            print!("{}", ascii as char)
        }

        self.previous_write = new
    }
}