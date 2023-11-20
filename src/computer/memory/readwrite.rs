use crate::{bits_to_index_length, bytes_to_store_bits};
use crate::un::U;

pub struct ReadWriteMemory<const N: usize, const M: usize> where [(); bytes_to_store_bits!(N)]: Sized {
    memory: [U<N>; M],
}

impl<const N: usize, const M: usize> ReadWriteMemory<N, M>
    where [(); bytes_to_store_bits!(N)]: Sized,
          [(); bytes_to_store_bits!(bits_to_index_length!(M))]: Sized  // Guarantee index type is representable
{
    pub fn new() -> Self {
        ReadWriteMemory {
            memory: [0u8.into(); M]
        }
    }

    pub fn with_values(values: [U<N>; M]) -> Self {
        ReadWriteMemory { memory: values }
    }

    pub fn read(&self, location: U<{ bits_to_index_length!(M) }>) -> U<N> {
        self.memory[usize::from(location)]
    }

    pub fn write(&mut self, location: U<{ bits_to_index_length!(M) }>, value: U<N>) {
        self.memory[usize::from(location)] = value;
    }
}
