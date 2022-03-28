pub struct ReadWriteMemory<T, const N: usize> {
    memory: [T; N],
}

impl<T: Copy + Default, const N: usize> ReadWriteMemory<T, N> {
    pub fn new() -> Self {
        ReadWriteMemory {
            memory: [T::default(); N],
        }
    }

    pub fn read<TAddr: Into<u8>>(&self, location: TAddr) -> T {
        self.memory[location.into() as usize]
    }

    pub fn write<TAddr: Into<u8>>(&mut self, location: TAddr, value: T) {
        self.memory[location.into() as usize] = value
    }
}
