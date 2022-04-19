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

#[cfg(test)]
mod tests {
    use super::*;

    fn initialize_alu() -> ArithmeticLogicUnit {
        let mut alu = ArithmeticLogicUnit::new();
        alu.accumulator.borrow_mut().store(RegisterType::new(5));
        alu
    }

    #[test]
    fn add() {
        let mut alu = initialize_alu();
        alu.add(RegisterType::new(4));
        assert_eq!(alu.accumulator.borrow().load(), RegisterType::new(9))
    }

    #[test]
    fn sub() {
        let mut alu = initialize_alu();
        alu.sub(RegisterType::new(4));
        assert_eq!(alu.accumulator.borrow().load(), RegisterType::new(1))
    }

    #[test]
    fn bor() {
        let mut alu = initialize_alu();
        // 5 is 0b101
        alu.bor(RegisterType::new(0b011));
        assert_eq!(alu.accumulator.borrow().load(), RegisterType::new(0b111))
    }

    #[test]
    fn and() {
        let mut alu = initialize_alu();
        // 5 is 0b101
        alu.and(RegisterType::new(0b011));
        assert_eq!(alu.accumulator.borrow().load(), RegisterType::new(0b001))
    }

    #[test]
    fn cmp_equal() {
        let alu = initialize_alu();
        let result = alu.cmp(RegisterType::new(5));
        assert_eq!(result, true)
    }

    #[test]
    fn cmp_not_equal() {
        let alu = initialize_alu();
        let result = alu.cmp(RegisterType::new(4));
        assert_eq!(result, false)
    }

    #[test]
    fn grt_equal() {
        let alu = initialize_alu();
        let result = alu.les(RegisterType::new(5));
        assert_eq!(result, false)
    }

    #[test]
    fn grt_less() {
        let alu = initialize_alu();
        let result = alu.les(RegisterType::new(4));
        assert_eq!(result, false)
    }

    #[test]
    fn grt_greater() {
        let alu = initialize_alu();
        let result = alu.les(RegisterType::new(6));
        assert_eq!(result, true)
    }

    #[test]
    fn les_equal() {
        let alu = initialize_alu();
        let result = alu.les(RegisterType::new(5));
        assert_eq!(result, false)
    }

    #[test]
    fn les_less() {
        let alu = initialize_alu();
        let result = alu.les(RegisterType::new(6));
        assert_eq!(result, true)
    }

    #[test]
    fn les_greater() {
        let alu = initialize_alu();
        let result = alu.les(RegisterType::new(4));
        assert_eq!(result, false)
    }
}
