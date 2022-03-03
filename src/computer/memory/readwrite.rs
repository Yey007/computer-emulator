pub struct ReadWriteMemory<T, TAddr: Into<usize>, const N: usize> {
    memory: [T; N],
}

impl<T: Copy + Default, TAddr: Into<usize>, const N: usize> ReadWriteMemory<T, TAddr, N> {
    pub fn new() -> Self {
        ReadWriteMemory {
            memory: [T::default(); N],
        }
    }

    pub fn read(&self, location: TAddr) -> T {
        self.memory[location]
    }

    pub fn write(&mut self, location: TAddr, value: T) {
        self.memory[location] = value
    }
}
