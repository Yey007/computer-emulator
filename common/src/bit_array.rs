use crate::bytes_to_store_bits;

/// An array storing a given number of bits `N`, indexable per bit.
#[derive(Debug, Copy, Clone)]
pub struct BitArray<const N: usize> where [(); bytes_to_store_bits!(N)]: Sized {
    /// The inner array. The `N`th bit and after is junk data and may be anything.
    inner_array: [u8; bytes_to_store_bits!(N)],
}

impl<const N: usize> BitArray<N> where [(); bytes_to_store_bits!(N)]: Sized {
    /// Returns a `BitArray` with all bits set to zero.
    pub fn zeroes() -> Self {
        BitArray {
            inner_array: [0u8; bytes_to_store_bits!(N)]
        }
    }

    /// Returns a `BitArray` with all bits set to one.
    pub fn ones() -> Self {
        BitArray {
            inner_array: [u8::MAX; bytes_to_store_bits!(N)],
        }
    }

    /// Returns a `BitArray` with the bits from the given array. The `N`th bit and after is ignored.
    ///
    /// # Arguments
    ///
    /// * `value`: The array.
    ///
    /// returns: BitArray<{ N }>
    pub fn from_array(value: [u8; bytes_to_store_bits!(N)]) -> Self {
        BitArray {
            inner_array: value
        }
    }

    /// Returns an array with the bits from this `BitArray`.
    ///
    /// returns: [u8; bytes_to_store_bits!(N)]
    pub fn to_array(&self) -> [u8; bytes_to_store_bits!(N)] {
        // We have to break the illusion here and make sure no bits other than those within N are set.
        change_bits_true(self.inner_array)
    }

    /// Returns the length (in bits) of this array.
    pub fn len(&self) -> usize {
        N
    }

    /// Gets a given bit in the bit array.
    ///
    /// # Arguments
    ///
    /// * `index`: The bit to get. 0-indexed and must be less than `N`.
    ///
    /// returns: bool
    pub fn get(&self, index: usize) -> bool {
        assert!(index < N);
        let array_index = index / 8;
        let value = self.inner_array[array_index];
        get_bit(value, (index % 8) as u8)
    }

    /// Sets a given bit in the bit array.
    ///
    /// # Arguments
    ///
    /// * `index`: The bit to set. 0-indexed and must be less than `N`.
    /// * `high`: Whether to set the bit high or low.
    ///
    /// returns: ()
    pub fn set(&mut self, index: usize, high: bool) {
        assert!(index < N);
        let array_index = index / 8;
        let bit_index = (index % 8) as u8;
        self.inner_array[array_index] = set_bit(self.inner_array[array_index], bit_index, high)
    }

    /// Returns a new bit array of the given size, truncating or extending this one.
    ///
    /// # Type Arguments
    ///
    /// * `TO`: The new size.
    ///
    /// returns: BitArray<{ M }>
    pub fn change_bits<const TO: usize>(self) -> BitArray<TO> where [(); bytes_to_store_bits!(TO)]: Sized
    {
        let mut out = [0u8; bytes_to_store_bits!(TO)];
        if TO > N {
            out[..self.inner_array.len()].copy_from_slice(&self.inner_array);
        } else {
            out[..].copy_from_slice(&self.inner_array[..bytes_to_store_bits!(TO)]);
        }

        BitArray {
            inner_array: out
        }
    }

    pub fn iter(&self) -> Iter<N> {
        Iter {
            forward_index: 0,
            backward_index: N - 1,
            backward_limit_reached: false,
            array: &self,
        }
    }

    pub fn map<F: FnMut(bool) -> bool>(mut self, mut f: F) -> BitArray<N> {
        for i in 0..self.len() {
            let initial = self.get(i);
            self.set(i, f(initial))
        }
        self
    }
}

impl<const N: usize> PartialEq for BitArray<N> where [(); bytes_to_store_bits!(N)]: Sized {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..N {
            if self.get(i) != other.get(i) {
                return false;
            }
        }
        true
    }
}

impl<const N: usize> Eq for BitArray<N> where [(); bytes_to_store_bits!(N)]: Sized {}

pub struct Iter<'a, const N: usize> where [(); bytes_to_store_bits!(N)]: Sized {
    forward_index: usize,
    backward_index: usize,
    backward_limit_reached: bool,
    array: &'a BitArray<N>,
}

impl<const N: usize> Iterator for Iter<'_, N> where [(); bytes_to_store_bits!(N)]: Sized {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.forward_index > self.backward_index || self.backward_limit_reached {
            return None;
        }

        let result = Some(self.array.get(self.forward_index));
        self.forward_index += 1;
        result
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.backward_index < self.forward_index || self.backward_limit_reached {
            return (0, Some(0));
        }

        let size = self.backward_index - self.forward_index + 1;
        (size, Some(size))
    }
}

impl<const N: usize> DoubleEndedIterator for Iter<'_, N> where [(); bytes_to_store_bits!(N)]: Sized {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.backward_index < self.forward_index || self.backward_limit_reached {
            return None;
        }

        let result = Some(self.array.get(self.backward_index));
        let option = self.backward_index.checked_sub(1);
        match option {
            Some(v) => self.backward_index = v,
            None => self.backward_limit_reached = true
        }

        result
    }
}

impl<const N: usize> ExactSizeIterator for Iter<'_, N> where [(); bytes_to_store_bits!(N)]: Sized {}

/// Returns whether the given 0-indexed bit is set in the given value.
///
/// # Arguments
///
/// * `value`: The value to check.
/// * `bit`: The (0-indexed) bit of the value to check. Must be less than 8.
///
/// returns: bool
fn get_bit(mut value: u8, bit: u8) -> bool {
    assert!(bit < 8);
    let mask = 0b00000001 << bit;
    value &= mask;
    value >>= bit;
    value == 1
}

/// Sets the given bit high or low in the given value and returns the new value.
///
/// # Arguments
///
/// * `value`: The value to set the bit in.
/// * `bit`: The (0-indexed) bit to set. Must be less than 8.
/// * `high`: Whether to set the bit high (to 1) or low (to 0).
///
/// returns: u8
fn set_bit(value: u8, bit: u8, high: bool) -> u8 {
    assert!(bit < 8);
    let set = 0b00000001 << bit;
    if high {
        value | set
    } else {
        value & !set
    }
}

/// Truncates or extends the given array of `FROM` bits to create an array storing `TO` bits.
///
/// # Type Arguments
///
/// * `FROM`: The number of bits stored in the input array.
/// * `TO`: The number of bits stored in the output array.
///
/// # Arguments
///
/// * `array`: The array to truncate or extend.
///
/// returns: [u8; bytes_to_store_bits!(N)]
fn change_bits_true<const FROM: usize, const TO: usize>(array: [u8; bytes_to_store_bits!(FROM)]) -> [u8; bytes_to_store_bits!(TO)] {
    // Make right length result array
    let mut out = [0u8; bytes_to_store_bits!(TO)];
    if TO > FROM {
        out[..array.len()].copy_from_slice(&array);
    } else {
        out[..].copy_from_slice(&array[..bytes_to_store_bits!(TO)]);
        let last = out.last_mut().unwrap();
        let shift = bytes_to_store_bits!(TO) * 8 - TO;
        assert!(shift < 8);
        *last &= 0b11111111 >> shift;
    }
    out
}

mod tests {
    use super::*;

    #[test]
    fn get_bit_test() {
        let n = 0b01101001u8;

        assert_eq!(get_bit(n, 7), false);
        assert_eq!(get_bit(n, 5), true);
        assert_eq!(get_bit(n, 1), false);
        assert_eq!(get_bit(n, 0), true);
    }

    #[test]
    fn set_bit_test() {
        let n = 0b01101001u8;

        assert_eq!(set_bit(n, 7, true), 0b11101001);
        assert_eq!(set_bit(n, 7, false), 0b01101001);

        assert_eq!(set_bit(n, 5, false), 0b01001001);
        assert_eq!(set_bit(n, 5, true), 0b01101001);

        assert_eq!(set_bit(n, 0, false), 0b01101000);
        assert_eq!(set_bit(n, 0, true), 0b01101001);
    }

    #[test]
    fn change_bits_true_test() {
        let a = [0b11101000u8, 0b11001101u8];

        let a2 = change_bits_true::<16, 18>(a);
        assert_eq!(a2, [0b11101000u8, 0b11001101u8, 0u8]);

        let a3 = change_bits_true::<16, 10>(a);
        assert_eq!(a3, [0b11101000u8, 0b00000001u8]);

        let a4 = change_bits_true::<16, 5>(a);
        assert_eq!(a4, [0b00001000u8]);

        let a5 = change_bits_true::<13, 13>(a);
        assert_eq!(a5, [0b11101000u8, 0b00001101u8]);
    }

    #[test]
    fn from_to_array_test() {
        let a = BitArray::<13>::from_array([0b11101000u8, 0b11001101u8]);

        assert_eq!(a.to_array(), [0b11101000u8, 0b00001101u8]);
    }

    #[test]
    fn get_test() {
        let a = BitArray::<13>::from_array([0b11101000u8, 0b00001101u8]);

        assert_eq!(a.get(0), false);
        assert_eq!(a.get(1), false);
        assert_eq!(a.get(2), false);
        assert_eq!(a.get(3), true);
        assert_eq!(a.get(4), false);
        assert_eq!(a.get(5), true);
        assert_eq!(a.get(6), true);
        assert_eq!(a.get(7), true);
        assert_eq!(a.get(8), true);
        assert_eq!(a.get(9), false);
        assert_eq!(a.get(10), true);
        assert_eq!(a.get(11), true);
        assert_eq!(a.get(12), false);
    }

    #[test]
    fn set_test() {
        let mut a = BitArray::<13>::from_array([0b11101000u8, 0b00001101u8]);

        a.set(0, false);
        assert_eq!(a, BitArray::<13>::from_array([0b11101000u8, 0b00001101u8]));

        a.set(0, true);
        assert_eq!(a, BitArray::<13>::from_array([0b11101001u8, 0b00001101u8]));

        a.set(11, true);
        assert_eq!(a, BitArray::<13>::from_array([0b11101001u8, 0b00001101u8]));

        a.set(11, false);
        assert_eq!(a, BitArray::<13>::from_array([0b11101001u8, 0b00000101u8]));
    }

    #[test]
    fn change_bits_test() {
        let a = BitArray::<13>::from_array([0b11101000u8, 0b00001101u8]);

        let a2 = a.change_bits::<16>();
        assert_eq!(a2, BitArray::<16>::from_array([0b11101000u8, 0b00001101u8]));

        let a3 = a.change_bits::<10>();
        assert_eq!(a3, BitArray::<10>::from_array([0b11101000u8, 0b00000001u8]));

        let a4 = a.change_bits::<5>();
        assert_eq!(a4, BitArray::<5>::from_array([0b00001000u8]));

        let a5 = a.change_bits::<13>();
        assert_eq!(a5, BitArray::<13>::from_array([0b11101000u8, 0b00001101u8]));
    }

    #[test]
    fn equality() {
        let a = BitArray::<13>::from_array([0b11101000u8, 0b00001101u8]);
        let b = BitArray::<13>::from_array([0b11101000u8, 0b00001101u8]);
        let c = BitArray::<13>::from_array([0b11101010u8, 0b00001100u8]);
        let d = BitArray::<13>::from_array([0b11101000u8, 0b00000100u8]);

        assert_eq!(a, b);
        assert_eq!(b, a);

        assert_ne!(a, c);
        assert_ne!(c, a);

        assert_ne!(a, d);
        assert_ne!(d, a);
    }

    #[test]
    fn iterator() {
        let a = BitArray::<12>::from_array([0b00101110u8, 0b00001000u8]);

        let forward: Vec<bool> = a.iter().collect();
        assert_eq!(forward, vec![false, true, true, true, false, true, false, false, false, false, false, true]);

        let backward: Vec<bool> = a.iter().rev().collect();
        assert_eq!(backward, vec![true, false, false, false, false, false, true, false, true, true, true, false]);

        let mut in_the_middle = a.iter();

        assert_eq!(in_the_middle.next(), Some(false));
        assert_eq!(in_the_middle.next(), Some(true));
        assert_eq!(in_the_middle.next(), Some(true));
        assert_eq!(in_the_middle.next(), Some(true));
        assert_eq!(in_the_middle.next(), Some(false));

        assert_eq!(in_the_middle.next_back(), Some(true));
        assert_eq!(in_the_middle.next_back(), Some(false));
        assert_eq!(in_the_middle.next_back(), Some(false));
        assert_eq!(in_the_middle.next_back(), Some(false));
        assert_eq!(in_the_middle.next_back(), Some(false));
        assert_eq!(in_the_middle.next_back(), Some(false));
        assert_eq!(in_the_middle.next_back(), Some(true));

        assert_eq!(in_the_middle.next(), None);
        assert_eq!(in_the_middle.next_back(), None);
    }

    #[test]
    fn iterator_size_hint() {
        let a = BitArray::<5>::from_array([0b00001101u8]);

        let mut iter = a.iter();
        assert_eq!(iter.size_hint(), (5, Some(5)));

        iter.next();
        iter.next();

        assert_eq!(iter.size_hint(), (3, Some(3)));

        iter.next_back();
        iter.next_back();

        assert_eq!(iter.size_hint(), (1, Some(1)));

        iter.next();

        assert_eq!(iter.size_hint(), (0, Some(0)));
    }

    #[test]
    fn map() {
        let a = BitArray::<5>::from_array([0b00001101u8]);

        let b = a.map(|b| !b);

        assert_eq!(b, BitArray::<5>::from_array([0b11110010u8]));
    }
}
