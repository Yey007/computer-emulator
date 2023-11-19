use std::ops::Add;
use const_guards::guard;

#[guard(N > 0)]
pub struct U<const N: usize> {
    value: [u8; (N - 1) / 8 + 1],
}

impl<const N: usize> U<N> {
    pub fn new(value: [u8; (N - 1) / 8 + 1]) -> Self <> {
        U {
            value
        }
    }
}

impl<const N: usize> Add for U<N> {
    type Output = U<{ N + 1 }>;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

macro_rules! impl_from {
    ($un:ty) => {
        impl<const N: usize> From<$un> for U<N> {
            fn from(value: $un) -> Self {
                let mut arr = [0; N];
                let bytes = <$un>::to_le_bytes(value);
                arr[..bytes.len()].clone_from_slice(&bytes);
                U::new(arr)
            }
        }
    };
}

macro_rules! impl_into {
    ($un:ty) => {
        #[guard(N <= $un::BITS as usize)]
        impl<const N: usize> Into<$un> for U<N> {
            fn into(self) -> $un {
                <$un>::from_le_bytes(self.value)
            }
        }
    };
}

impl_from!(u8);
impl_from!(u16);
impl_from!(u32);
impl_from!(u64);
impl_from!(u128);

impl_into!(u8);
impl_into!(u16);
impl_into!(u32);
impl_into!(u64);
impl_into!(u128);
