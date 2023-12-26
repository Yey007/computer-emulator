use std::cell::{RefCell};
use std::rc::Rc;
use crate::bytes_to_store_bits;
use crate::connectable::Connectable;
use crate::un::U;

pub struct Spliter<const N: usize, const M: usize>
    where [(); bytes_to_store_bits!(N)]: Sized,
          [(); bytes_to_store_bits!(M)]: Sized,
          [(); bytes_to_store_bits!({ N + M })]: Sized
{
    combined_connection: Option<Rc<RefCell<dyn Connectable<{ N + M }>>>>,
    low_end_connection: Option<Rc<RefCell<dyn Connectable<N>>>>,
    high_end_connection: Option<Rc<RefCell<dyn Connectable<M>>>>,
}

impl<const N: usize, const M: usize> Spliter<N, M>
    where [(); bytes_to_store_bits!(N)]: Sized,
          [(); bytes_to_store_bits!(M)]: Sized,
          [(); bytes_to_store_bits!({ N + M })]: Sized
{
    pub fn new() -> Self <> {
        Spliter {
            combined_connection: None,
            low_end_connection: None,
            high_end_connection: None,
        }
    }

    pub fn as_low_end(&mut self) -> LowEnd<N, M> {
        LowEnd(self)
    }

    pub fn as_high_end(&mut self) -> HighEnd<N, M> {
        HighEnd(self)
    }
}

impl<const N: usize, const M: usize> Connectable<{ N + M }> for Spliter<N, M>
    where [(); bytes_to_store_bits!(N)]: Sized,
          [(); bytes_to_store_bits!(M)]: Sized,
          [(); bytes_to_store_bits!({ N + M })]: Sized
{
    fn pull_value(&self) -> U<{ N + M }> {
        let low_end = match &self.low_end_connection {
            Some(conn) => conn.borrow_mut().pull_value().change_bits(),
            None => 0u8.into()
        };

        let high_end = match &self.high_end_connection {
            Some(conn) => conn.borrow_mut().pull_value().change_bits(),
            None => 0u8.into()
        };

        // still can't convert from usize :(
        (high_end << (N as u128).into()) | low_end
    }

    fn connect_to(&mut self, other: Rc<RefCell<dyn Connectable<{ N + M }>>>) {
        self.combined_connection = Some(other)
    }
}

pub struct LowEnd<'a, const N: usize, const M: usize>(&'a mut Spliter<N, M>)
    where [(); bytes_to_store_bits!(N)]: Sized,
          [(); bytes_to_store_bits!(M)]: Sized,
          [(); bytes_to_store_bits!({ N + M })]: Sized;

impl<const N: usize, const M: usize> Connectable<N> for LowEnd<'_, N, M>
    where [(); bytes_to_store_bits!(N)]: Sized,
          [(); bytes_to_store_bits!(M)]: Sized,
          [(); bytes_to_store_bits!({ N + M })]: Sized
{
    fn pull_value(&self) -> U<N> {
        if let Some(conn) = &self.0.high_end_connection {
            conn.borrow_mut().pull_value().change_bits()
        } else {
            0u8.into()
        }
    }

    fn connect_to(&mut self, other: Rc<RefCell<dyn Connectable<N>>>) {
        self.0.low_end_connection = Some(other)
    }
}

pub struct HighEnd<'a, const N: usize, const M: usize>(&'a mut Spliter<N, M>)
    where [(); bytes_to_store_bits!(N)]: Sized,
          [(); bytes_to_store_bits!(M)]: Sized,
          [(); bytes_to_store_bits!({ N + M })]: Sized;

impl<const N: usize, const M: usize> Connectable<M> for HighEnd<'_, N, M>
    where [(); bytes_to_store_bits!(N)]: Sized,
          [(); bytes_to_store_bits!(M)]: Sized,
          [(); bytes_to_store_bits!({ N + M })]: Sized
{
    fn pull_value(&self) -> U<M> {
        if let Some(conn) = &self.0.high_end_connection {
            let shifted = conn.borrow_mut().pull_value() >> (N as u128).into();
            shifted.change_bits()
        } else {
            0u8.into()
        }
    }

    fn connect_to(&mut self, other: Rc<RefCell<dyn Connectable<M>>>) {
        self.0.high_end_connection = Some(other)
    }
}
