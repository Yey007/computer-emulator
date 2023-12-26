use std::cell::{RefCell};
use std::rc::Rc;
use crate::bytes_to_store_bits;
use crate::connectable::Connectable;
use crate::notif_cell::NotifCell;
use crate::un::U;

pub struct Spliter<const N: usize, const M: usize>
    where [(); bytes_to_store_bits!(N)]: Sized,
          [(); bytes_to_store_bits!(M)]: Sized,
          [(); bytes_to_store_bits!({ N + M })]: Sized
{
    combined: Rc<NotifCell<U<{ N + M }>>>,
    low_end: Rc<NotifCell<U<N>>>,
    high_end: Rc<NotifCell<U<M>>>
}

impl<const N: usize, const M: usize> Spliter<N, M>
    where [(); bytes_to_store_bits!(N)]: Sized,
          [(); bytes_to_store_bits!(M)]: Sized,
          [(); bytes_to_store_bits!({ N + M })]: Sized
{
    pub fn new() -> Self <> {
        Spliter {
            combined: Rc::new(NotifCell::new(0u8.into())),
            low_end: Rc::new(NotifCell::new(0u8.into())),
            high_end: Rc::new(NotifCell::new(0u8.into()))
        }
    }

    pub fn as_low_end(&self) -> LowEnd<N, M> {
        LowEnd(&self)
    }

    pub fn as_high_end(&self) -> HighEnd<N, M> {
        HighEnd(&self)
    }
}

impl<const N: usize, const M: usize> Connectable<{ N + M }> for Spliter<N, M> {
    fn get_value_ref(&self) -> Rc<NotifCell<U<{ N + M }>>> {
        todo!()
    }

    fn connect_to(&mut self, other: &dyn Connectable<{ N + M }>) {
        self.combined = other.get_value_ref();
        self.combined.on_change()
    }
}

pub struct LowEnd<'a, const N: usize, const M: usize>(&'a Spliter<N, M>)
    where [(); bytes_to_store_bits!(N)]: Sized,
          [(); bytes_to_store_bits!(M)]: Sized,
          [(); bytes_to_store_bits!({ N + M })]: Sized;

pub struct HighEnd<'a, const N: usize, const M: usize>(&'a Spliter<N, M>)
    where [(); bytes_to_store_bits!(N)]: Sized,
          [(); bytes_to_store_bits!(M)]: Sized,
          [(); bytes_to_store_bits!({ N + M })]: Sized;
