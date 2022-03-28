use crate::computer::register::Register;
use std::borrow::{Borrow, BorrowMut};
use ux::u4;

pub struct ArithmeticLogicUnit {
    accumulator: Register<u4>,
}

impl ArithmeticLogicUnit {
    pub fn new() -> Self {
        ArithmeticLogicUnit {
            accumulator: Register::new(),
        }
    }

    pub fn accumulator(&self) -> &Register<u4> {
        self.accumulator.borrow()
    }

    pub fn accumulator_mut(&mut self) -> &mut Register<u4> {
        self.accumulator.borrow_mut()
    }

    pub fn add(&mut self, value: u4) {
        let current = self.accumulator.load();
        self.accumulator.store(current + value)
    }

    pub fn sub(&mut self, value: u4) {
        let current = self.accumulator.load();
        self.accumulator.store(current - value)
    }

    pub fn bor(&mut self, value: u4) {
        let current = self.accumulator.load();
        self.accumulator.store(current | value)
    }

    pub fn and(&mut self, value: u4) {
        let current = self.accumulator.load();
        self.accumulator.store(current & value)
    }

    pub fn cmp(&self, value: u4) -> bool {
        let current = self.accumulator.load();
        current == value
    }

    pub fn grt(&self, value: u4) -> bool {
        let current = self.accumulator.load();
        current > value
    }

    pub fn les(&self, value: u4) -> bool {
        let current = self.accumulator.load();
        current < value
    }
}
