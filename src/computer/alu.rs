use crate::computer::register::Register;
use crate::types::WorkingType;
use std::borrow::{Borrow, BorrowMut};

pub struct ArithmeticLogicUnit {
    accumulator: Register<WorkingType>,
}

impl ArithmeticLogicUnit {
    pub fn new() -> Self {
        ArithmeticLogicUnit {
            accumulator: Register::new(),
        }
    }

    pub fn accumulator(&self) -> &Register<WorkingType> {
        self.accumulator.borrow()
    }

    pub fn accumulator_mut(&mut self) -> &mut Register<WorkingType> {
        self.accumulator.borrow_mut()
    }

    pub fn add(&mut self, value: WorkingType) {
        let current = self.accumulator.load();
        self.accumulator.store(current + value)
    }

    pub fn sub(&mut self, value: WorkingType) {
        let current = self.accumulator.load();
        self.accumulator.store(current - value)
    }

    pub fn bor(&mut self, value: WorkingType) {
        let current = self.accumulator.load();
        self.accumulator.store(current | value)
    }

    pub fn and(&mut self, value: WorkingType) {
        let current = self.accumulator.load();
        self.accumulator.store(current & value)
    }

    pub fn cmp(&self, value: WorkingType) -> bool {
        let current = self.accumulator.load();
        current == value
    }

    pub fn grt(&self, value: WorkingType) -> bool {
        let current = self.accumulator.load();
        current > value
    }

    pub fn les(&self, value: WorkingType) -> bool {
        let current = self.accumulator.load();
        current < value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn initialize_alu() -> ArithmeticLogicUnit {
        let mut alu = ArithmeticLogicUnit::new();
        alu.accumulator.borrow_mut().store(WorkingType::new(5));
        alu
    }

    #[test]
    fn add() {
        let mut alu = initialize_alu();
        alu.add(WorkingType::new(4));
        assert_eq!(alu.accumulator.borrow().load(), WorkingType::new(9))
    }

    #[test]
    fn sub() {
        let mut alu = initialize_alu();
        alu.sub(WorkingType::new(4));
        assert_eq!(alu.accumulator.borrow().load(), WorkingType::new(1))
    }

    #[test]
    fn bor() {
        let mut alu = initialize_alu();
        // 5 is 0b101
        alu.bor(WorkingType::new(0b011));
        assert_eq!(alu.accumulator.borrow().load(), WorkingType::new(0b111))
    }

    #[test]
    fn and() {
        let mut alu = initialize_alu();
        // 5 is 0b101
        alu.and(WorkingType::new(0b011));
        assert_eq!(alu.accumulator.borrow().load(), WorkingType::new(0b001))
    }

    #[test]
    fn cmp_equal() {
        let alu = initialize_alu();
        let result = alu.cmp(WorkingType::new(5));
        assert_eq!(result, true)
    }

    #[test]
    fn cmp_not_equal() {
        let alu = initialize_alu();
        let result = alu.cmp(WorkingType::new(4));
        assert_eq!(result, false)
    }

    #[test]
    fn grt_equal() {
        let alu = initialize_alu();
        let result = alu.les(WorkingType::new(5));
        assert_eq!(result, false)
    }

    #[test]
    fn grt_less() {
        let alu = initialize_alu();
        let result = alu.les(WorkingType::new(4));
        assert_eq!(result, false)
    }

    #[test]
    fn grt_greater() {
        let alu = initialize_alu();
        let result = alu.les(WorkingType::new(6));
        assert_eq!(result, true)
    }

    #[test]
    fn les_equal() {
        let alu = initialize_alu();
        let result = alu.les(WorkingType::new(5));
        assert_eq!(result, false)
    }

    #[test]
    fn les_less() {
        let alu = initialize_alu();
        let result = alu.les(WorkingType::new(6));
        assert_eq!(result, true)
    }

    #[test]
    fn les_greater() {
        let alu = initialize_alu();
        let result = alu.les(WorkingType::new(4));
        assert_eq!(result, false)
    }
}
