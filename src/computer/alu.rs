use crate::computer::register::Register;
use std::borrow::{Borrow, BorrowMut};
use ux::u4;

type RegisterType = u4;

pub struct ArithmeticLogicUnit {
    accumulator: Register<RegisterType>,
}

impl ArithmeticLogicUnit {
    pub fn new() -> Self {
        ArithmeticLogicUnit {
            accumulator: Register::new(),
        }
    }

    pub fn accumulator(&self) -> &Register<RegisterType> {
        self.accumulator.borrow()
    }

    pub fn accumulator_mut(&mut self) -> &mut Register<RegisterType> {
        self.accumulator.borrow_mut()
    }

    pub fn add(&mut self, value: RegisterType) {
        let current = self.accumulator.load();
        self.accumulator.store(current + value)
    }

    pub fn sub(&mut self, value: RegisterType) {
        let current = self.accumulator.load();
        self.accumulator.store(current - value)
    }

    pub fn bor(&mut self, value: RegisterType) {
        let current = self.accumulator.load();
        self.accumulator.store(current | value)
    }

    pub fn and(&mut self, value: RegisterType) {
        let current = self.accumulator.load();
        self.accumulator.store(current & value)
    }

    pub fn cmp(&self, value: RegisterType) -> bool {
        let current = self.accumulator.load();
        current == value
    }

    pub fn grt(&self, value: RegisterType) -> bool {
        let current = self.accumulator.load();
        current > value
    }

    pub fn les(&self, value: RegisterType) -> bool {
        let current = self.accumulator.load();
        current < value
    }
}
