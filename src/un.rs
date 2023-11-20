use std::cmp::Ordering;
use std::ops::{Add, BitAnd, BitOr, Sub};

#[macro_export]
macro_rules! bytes_to_store_bits {
    ($n:expr) => {
        ($n - 1) / 8 + 1
    };
}


#[derive(Debug, Copy, Clone)]
pub struct U<const N: usize> where [(); bytes_to_store_bits!(N)]: Sized {
    value: [u8; bytes_to_store_bits!(N)],
}


impl<const N: usize> U<N> where [(); bytes_to_store_bits!(N)]: Sized {
    pub fn new(value: [u8; bytes_to_store_bits!(N)]) -> Self <> {
        U {
            value
        }
    }
}

impl<const N: usize> Add for U<N> where [(); bytes_to_store_bits!(N)]: Sized {
    type Output = U<N>;  // Add with overflow

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<const N: usize> Sub for U<N> where [(); bytes_to_store_bits!(N)]: Sized {
    type Output = U<N>;

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<const N: usize> BitAnd for U<N> where [(); bytes_to_store_bits!(N)]: Sized {
    type Output = U<N>;

    fn bitand(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<const N: usize> BitOr for U<N> where [(); bytes_to_store_bits!(N)]: Sized {
    type Output = U<N>;

    fn bitor(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<const N: usize> Ord for U<N> where [(); bytes_to_store_bits!(N)]: Sized {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!()
    }
}

impl<const N: usize> PartialOrd<Self> for U<N> where [(); bytes_to_store_bits!(N)]: Sized {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!()
    }
}

impl<const N: usize> PartialEq<Self> for U<N> where [(); bytes_to_store_bits!(N)]: Sized {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl<const N: usize> Eq for U<N> where [(); bytes_to_store_bits!(N)]: Sized {}


macro_rules! impl_from_primitive {
    ($un:ty) => {
        impl<const N: usize> From<$un> for U<N> where [(); bytes_to_store_bits!(N)]: Sized {
            fn from(value: $un) -> Self {
                let mut arr = [0; bytes_to_store_bits!(N)];
                let bytes = <$un>::to_le_bytes(value);
                arr[..bytes.len()].clone_from_slice(&bytes);
                U::new(arr)
            }
        }
    };
}

macro_rules! impl_to_primitive {
    ($un:ty) => {
        impl<const N: usize> From<U<N>> for $un where [(); bytes_to_store_bits!(N)]: Sized {
            fn from(value: U<N>) -> Self {
                let arr = &value.value[..std::mem::size_of::<$un>()];
                <$un>::from_le_bytes(arr.try_into().unwrap())  // hypothetically can never fail due to guard
            }
        }
    };
}

impl_from_primitive!(u8);
impl_from_primitive!(u16);
impl_from_primitive!(u32);
impl_from_primitive!(u64);
impl_from_primitive!(u128);
impl_from_primitive!(usize);

impl_to_primitive!(u8);
impl_to_primitive!(u16);
impl_to_primitive!(u32);
impl_to_primitive!(u64);
impl_to_primitive!(u128);
impl_to_primitive!(usize);
