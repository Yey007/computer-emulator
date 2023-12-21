// use std::cell::{RefCell};
// use std::rc::Rc;
// use crate::bytes_to_store_bits;
// use crate::connectable::Connectable;
// use crate::un::U;
//
// pub struct Spliter<const N: usize, const M: usize>
//     where [(); bytes_to_store_bits!(N)]: Sized,
//           [(); bytes_to_store_bits!(M)]: Sized,
//           [(); bytes_to_store_bits!({ N + M })]: Sized
// {
//     combined: Rc<RefCell<U<{ N + M }>>>,
//     low_end: Rc<RefCell<U<N>>>,
//     high_end: Rc<RefCell<U<M>>>
// }
//
// impl<const N: usize, const M: usize> Spliter<N, M>
//     where [(); bytes_to_store_bits!(N)]: Sized,
//           [(); bytes_to_store_bits!(M)]: Sized,
//           [(); bytes_to_store_bits!({ N + M })]: Sized
// {
//     pub fn new() -> Self <> {
//         Spliter {
//             combined: Rc::new(RefCell::new(0u8.into())),
//             low_end: Rc::new(RefCell::new(0u8.into())),
//             high_end: Rc::new(RefCell::new(0u8.into()))
//         }
//     }
//
//     pub fn as_low_end(&self) -> LowEnd<N, M> {
//         LowEnd(&self)
//     }
//
//     pub fn as_high_end(&self) -> HighEnd<N, M> {
//         HighEnd(&self)
//     }
// }
//
// pub struct LowEnd<'a, const N: usize, const M: usize>(&'a Spliter<N, M>)
//     where [(); bytes_to_store_bits!(N)]: Sized,
//           [(); bytes_to_store_bits!(M)]: Sized,
//           [(); bytes_to_store_bits!({ N + M })]: Sized;
//
// impl<'a, const N: usize, const M: usize> Connectable<N> for LowEnd<'a, N, M>
//     where [(); bytes_to_store_bits!(N)]: Sized,
//           [(); bytes_to_store_bits!(M)]: Sized,
//           [(); bytes_to_store_bits!({ N + M })]: Sized
// {
//     fn get_value_ref(&self) -> Rc<RefCell<U<N>>> {
//         self.0.low_end.clone()
//     }
// }
//
// pub struct HighEnd<'a, const N: usize, const M: usize>(&'a Spliter<N, M>)
//     where [(); bytes_to_store_bits!(N)]: Sized,
//           [(); bytes_to_store_bits!(M)]: Sized,
//           [(); bytes_to_store_bits!({ N + M })]: Sized;
//
// impl<'a, const N: usize, const M: usize> Connectable<M> for HighEnd<'a, N, M>
//     where [(); bytes_to_store_bits!(N)]: Sized,
//           [(); bytes_to_store_bits!(M)]: Sized,
//           [(); bytes_to_store_bits!({ N + M })]: Sized
// {
//     fn get_value_ref(&self) -> Rc<RefCell<U<M>>> {
//         self.0.high_end.clone()
//     }
// }
//
// impl<const N: usize, const M: usize> Connectable<{ N + M }> for Spliter<N, M>
//     where [(); bytes_to_store_bits!(N)]: Sized,
//           [(); bytes_to_store_bits!(M)]: Sized,
//           [(); bytes_to_store_bits!({ N + M })]: Sized
// {
//     fn get_value_ref(&self) -> Rc<RefCell<U<{ N + M }>>> {
//         self.combined.clone()
//     }
// }
