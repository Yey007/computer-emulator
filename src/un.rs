use std::cmp::{min, Ordering};
use std::ops::{Add, AddAssign, BitAnd, BitOr, Shl, Shr, Sub, SubAssign};

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
    pub fn new() -> Self <> {
        U {
            value: [0u8; bytes_to_store_bits!(N)]
        }
    }

    pub fn with_value(value: [u8; bytes_to_store_bits!(N)]) -> Self <> {
        U {
            value: change_bits(value)
        }
    }

    pub fn max() -> Self <> {
        U {
            value: change_bits([u8::MAX; bytes_to_store_bits!(N)])
        }
    }

    pub fn change_bits<const M: usize>(&self) -> U<M> where [(); bytes_to_store_bits!(M)]: Sized {
        U {
            value: change_bits(self.value)
        }
    }
}

// Important difference between U<N> and standard types: U<N> over/underflows by default.
impl<const N: usize> Add for U<N> where [(); bytes_to_store_bits!(N)]: Sized {
    type Output = U<N>;  // Add with overflow

    fn add(self, rhs: Self) -> Self::Output {
        let mut out_arr = [0u8; bytes_to_store_bits!(N)];

        let mut carry = 0u8;
        for (i, (&n1, n2)) in self.value.iter().zip(rhs.value).enumerate() {
            let sum = n1 as u16 + n2 as u16 + carry as u16;
            out_arr[i] = sum as u8;
            carry = (sum >> 8) as u8
        }

        U::with_value(out_arr)
    }
}

impl<const N: usize> AddAssign for U<N> where [(); bytes_to_store_bits!(N)]: Sized {
    fn add_assign(&mut self, rhs: Self) {
        // TODO: efficiency
        *self = *self + rhs
    }
}

impl<const N: usize> Sub for U<N> where [(); bytes_to_store_bits!(N)]: Sized {
    type Output = U<N>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut out_arr = [0u8; bytes_to_store_bits!(N)];

        let mut borrow_from_next = false;
        for (i, (&n1, n2)) in self.value.iter().zip(rhs.value).enumerate() {
            // We can basically think of n1, n2 as base-256 digits
            let mut n1_i32 = n1 as i32;
            let n2_i32 = n2 as i32;

            if borrow_from_next {
                n1_i32 -= 1;
            }

            borrow_from_next = false;

            if n1_i32 < n2_i32 {
                borrow_from_next = true;
                n1_i32 += u8::MAX as i32 + 1;
            }

            debug_assert!(0 <= n1_i32 - n2_i32);
            debug_assert!(n1_i32 - n2_i32 <= u8::MAX as i32);

            out_arr[i] = (n1_i32 - n2_i32) as u8;
        }

        U::with_value(out_arr)
    }
}

impl<const N: usize> SubAssign for U<N> where [(); bytes_to_store_bits!(N)]: Sized {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl<const N: usize> BitAnd for U<N> where [(); bytes_to_store_bits!(N)]: Sized {
    type Output = U<N>;

    fn bitand(self, rhs: Self) -> Self::Output {
        let out_vec: Vec<u8> = self.value.iter().zip(rhs.value).map(|(&n1, n2)| n1 & n2).collect();
        let out_arr: [u8; bytes_to_store_bits!(N)] = out_vec.try_into().unwrap();
        U::with_value(out_arr)
    }
}

impl<const N: usize> BitOr for U<N> where [(); bytes_to_store_bits!(N)]: Sized {
    type Output = U<N>;

    fn bitor(self, rhs: Self) -> Self::Output {
        let out_vec: Vec<u8> = self.value.iter().zip(rhs.value).map(|(&n1, n2)| n1 | n2).collect();
        let out_arr: [u8; bytes_to_store_bits!(N)] = out_vec.try_into().unwrap();
        U::with_value(out_arr)
    }
}

trait ZeroedShift {
    fn zeroed_shr(self, rhs: u32) -> Self;
    fn zeroed_shl(self, rhs: u32) -> Self;
}

impl ZeroedShift for u8 {
    fn zeroed_shr(self, rhs: u32) -> Self {
        self.checked_shr(rhs).unwrap_or(0)
    }

    fn zeroed_shl(self, rhs: u32) -> Self {
        self.checked_shl(rhs).unwrap_or(0)
    }
}

impl<const N: usize> Shr for U<N> where [(); bytes_to_store_bits!(N)]: Sized {
    type Output = U<N>;

    fn shr(self, rhs: Self) -> Self::Output {
        let mut arr = self.value;
        let mut shifts_so_far: Self = 0u8.into();
        while rhs > shifts_so_far {
            let shifts_to_do: u32 = min(rhs - shifts_so_far, 8u8.into()).into();
            for i in 0..(arr.len() - 1) {
                // get bottom "shifts_to_do" bits of next higher byte -- this is what will be shifted out
                let shifted_out = arr[i + 1] & (0b11111111u8.zeroed_shr(8 - shifts_to_do));
                // shift the current bytes
                arr[i] = arr[i].zeroed_shr(shifts_to_do);
                // replace the upper "shifts_to_do" bits with the bits from "shifted_out"
                arr[i] |= shifted_out.zeroed_shl(8 - shifts_to_do);
            }
            // shift the highest byte
            arr[arr.len() - 1] = arr[arr.len() - 1].zeroed_shr(shifts_to_do);

            shifts_so_far += shifts_to_do.into()
        }

        U::with_value(arr)
    }
}

impl<const N: usize> Shl for U<N> where [(); bytes_to_store_bits!(N)]: Sized {
    type Output = U<N>;

    fn shl(self, rhs: Self) -> Self::Output {
        let mut arr = self.value;
        let mut shifts_so_far: Self = 0u8.into();
        while rhs > shifts_so_far {
            let shifts_to_do: u32 = min(rhs - shifts_so_far, 8u8.into()).into();
            for i in (1..arr.len()).rev() {
                // get top "shifts_to_do" bits of next lower byte -- this is what will be shifted out
                let shifted_out = arr[i - 1] & (0b11111111u8.zeroed_shl(8 - shifts_to_do));
                // shift the current byte
                arr[i] = arr[i].zeroed_shl(shifts_to_do);
                // replace the lower "shifts_to_do" bits with the bits from "shifted_out"
                arr[i] |= shifted_out.zeroed_shr(8 - shifts_to_do);
            }
            // shift the lowest byte
            arr[0] = arr[0].zeroed_shl(shifts_to_do);

            shifts_so_far += shifts_to_do.into()
        }

        U::with_value(arr)
    }
}

impl<const N: usize> Ord for U<N> where [(); bytes_to_store_bits!(N)]: Sized {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.value == other.value {
            Ordering::Equal
        } else {
            for (n1, n2) in self.value.iter().zip(other.value).rev() {
                if n1 > &n2 {
                    return Ordering::Greater;
                } else if n1 < &n2 {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        }
    }
}

impl<const N: usize> PartialOrd<Self> for U<N> where [(); bytes_to_store_bits!(N)]: Sized {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> PartialEq<Self> for U<N> where [(); bytes_to_store_bits!(N)]: Sized {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<const N: usize> Eq for U<N> where [(); bytes_to_store_bits!(N)]: Sized {}


macro_rules! impl_from_primitive {
    ($un:ty, $bits:expr) => {
        impl<const N: usize> From<$un> for U<N> where [(); bytes_to_store_bits!(N)]: Sized {
            fn from(value: $un) -> Self {
                let bytes = <$un>::to_le_bytes(value);
                U::with_value(change_bits::<$bits, N>(bytes))
            }
        }
    };
}

macro_rules! impl_to_primitive {
    ($un:ty, $bits:expr) => {
        impl<const N: usize> From<U<N>> for $un where [(); bytes_to_store_bits!(N)]: Sized {
            fn from(value: U<N>) -> Self {
                let bytes = change_bits::<N, $bits>(value.value);
                <$un>::from_le_bytes(bytes)
            }
        }
    };
}

// Passing the expression { ux::BITS as usize } fails type checking for little reason, even when
// the proper where bounds are included (to my knowledge). So we have to resort to this
impl_from_primitive!(u8, 8);
impl_from_primitive!(u16, 16);
impl_from_primitive!(u32, 32);
impl_from_primitive!(u64, 62);
impl_from_primitive!(u128, 128);
// Due to the above, this becomes impossible to implement for usize. Hopefully we don't need it.
// impl_from_primitive!(usize);

// Same here
impl_to_primitive!(u8, 8);
impl_to_primitive!(u16, 16);
impl_to_primitive!(u32, 32);
impl_to_primitive!(u64, 64);
impl_to_primitive!(u128, 128);
// impl_to_primitive!(usize);

fn change_bits<const M: usize, const N: usize>(array: [u8; bytes_to_store_bits!(M)]) -> [u8; bytes_to_store_bits!(N)] {
    // Make right length result array
    let mut out = [0u8; bytes_to_store_bits!(N)];
    if M < N {
        out[..array.len()].copy_from_slice(&array);
    } else {
        out[..].copy_from_slice(&array[..bytes_to_store_bits!(N)]);
        let last = out.last_mut().unwrap();
        let shift = bytes_to_store_bits!(N) * 8 - N;
        debug_assert!(shift < 8);
        *last &= 0b11111111 >> shift;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_primitive() {
        let a: U<128> = u128::MAX.into();
        assert_eq!(a.value, [u8::MAX; 128 / 8]);

        let b: U<16> = u32::MAX.into();
        assert_eq!(b.value, [u8::MAX, u8::MAX]);

        let c: U<12> = u16::MAX.into();
        assert_eq!(c.value, [u8::MAX, 0b00001111]);

        let d: U<4> = 2u8.into();
        assert_eq!(d.value, [2]);

        let e: U<9> = 10u8.into();
        assert_eq!(e.value, [0b00001010, 0]);

        let f: U<31> = u64::MAX.into();
        assert_eq!(f.value, [u8::MAX, u8::MAX, u8::MAX, 0b01111111])
    }

    #[test]
    fn to_primitive() {
        let a: U<128> = u128::MAX.into();
        assert_eq!(u128::from(a), u128::MAX)
    }

    #[test]
    fn equality() {
        let a: U<4> = 1u8.into();
        let b: U<4> = 1u8.into();
        let c: U<4> = 2u8.into();

        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(b, c);

        let d: U<128> = u128::MAX.into();
        let e: U<128> = u128::MAX.into();
        let f: U<128> = (u128::MAX - 10).into();

        assert_eq!(d, e);
        assert_ne!(d, f);
        assert_ne!(e, f);
    }

    #[test]
    fn inequality() {
        let a: U<8> = 5u8.into();
        let b: U<8> = 200u8.into();

        assert!(a < b);
        assert!(b > a);

        let c: U<130> = u128::MAX.into();
        let d: U<130> = (u128::MAX - 10).into();

        assert!(c > d);
        assert!(d < c);

        let e: U<10> = 10u8.into();
        let f: U<10> = 10u8.into();

        assert!(!(e > f));
        assert!(!(e < f));
    }

    #[test]
    fn add() {
        let a: U<4> = 8u8.into();
        let b: U<4> = 1u8.into();

        assert_eq!(a + b, 9u8.into());

        let c: U<128> = (9u128 * 10u128.pow(10)).into();
        let d: U<128> = (3u128 * 10u128.pow(10)).into();

        assert_eq!(c + d, (12u128 * 10u128.pow(10)).into());

        let f: U<4> = 0b1111u8.into();
        let g: U<4> = 0b1111u8.into();
        assert_eq!(f + g, 0b1110u8.into());

        let h: U<8> = u8::MAX.into();
        let i: U<8> = u8::MAX.into();
        assert_eq!(h + i, 0b11111110u8.into())
    }

    #[test]
    fn sub() {
        let a: U<4> = 8u8.into();
        let b: U<4> = 1u8.into();

        assert_eq!(a - b, 7u8.into());

        let c: U<128> = (9u128 * 10u128.pow(10)).into();
        let d: U<128> = (3u128 * 10u128.pow(10)).into();

        assert_eq!(c - d, (6u128 * 10u128.pow(10)).into());

        let e: U<9> = 0b000000000u16.into();
        let f: U<9> = 0b000000001u16.into();

        assert_eq!(e - f, 0b111111111u16.into());

        let h: U<15> = 0b1100111_00100001u16.into();
        let i: U<15> = 0b1101111_00100010u16.into();

        assert_eq!(h - i, 0b1110111_11111111u16.into());

        let j: U<16> = 0b01100111_00100001u16.into();
        let k: U<16> = 0b01101111_00100010u16.into();

        assert_eq!(u16::from(j - k), 0b01100111_00100001u16.overflowing_sub(0b01101111_00100010u16).0)
    }

    #[test]
    fn bit_and() {
        let a: U<4> = 0b1100u8.into();
        let b: U<4> = 0b0101u8.into();

        assert_eq!(a & b, 0b0100u8.into());

        let c: U<13> = 0b1_1000_1111_0111u16.into();
        let d: U<13> = 0b1_1010_0101_1100u16.into();

        assert_eq!(c & d, 0b1_1000_0101_0100u16.into())
    }

    #[test]
    fn bit_or() {
        let a: U<4> = 0b1100u8.into();
        let b: U<4> = 0b0101u8.into();

        assert_eq!(a | b, 0b1101u8.into());

        let c: U<13> = 0b1_1000_1111_0111u16.into();
        let d: U<13> = 0b1_1010_0101_1100u16.into();

        assert_eq!(c | d, 0b1_1010_1111_1111u16.into())
    }

    #[test]
    fn shr() {
        let a: U<8> = 0b01101001u8.into();
        assert_eq!(a >> 5u8.into(), 0b00000011u8.into());

        let b: U<12> = 0b0110_11000111u16.into();
        assert_eq!(b >> 3u8.into(), 0b0000_11011000u16.into());
        assert_eq!(b >> 9u8.into(), 0b0000_00000011u16.into());
    }

    #[test]
    fn shl() {
        let a: U<8> = 0b01101001u8.into();
        assert_eq!(a << 5u8.into(), 0b00100000u8.into());

        let b: U<12> = 0b0110_11000111u16.into();
        assert_eq!(b << 3u8.into(), 0b0110_00111000u16.into());
        assert_eq!(b << 9u8.into(), 0b1110_00000000u16.into());
    }
}
