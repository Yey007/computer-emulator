use crate::ux_extensions::extensions::{Decrement, Increment};
use ux::{u4, u6};

impl Increment for u4 {
    fn increment(mut self) {
        self = self + u4::new(1)
    }
}

impl Decrement for u4 {
    fn decrement(mut self) {
        self = self - u4::new(1)
    }
}

impl Increment for u6 {
    fn increment(mut self) {
        self = self + u6::new(1)
    }
}

impl Decrement for u6 {
    fn decrement(mut self) {
        self = self - u6::new(1)
    }
}
