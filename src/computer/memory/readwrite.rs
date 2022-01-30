pub struct ReadWriteMemory<T, const N: usize> {
    memory: [T; N],
}

impl<T: Copy, const N: usize> ReadWriteMemory<T, N> {
    pub fn new() -> Self {
       ReadWriteMemory { memory: [N] }
    }

    pub fn read(&self, location: usize) -> T {
        self.memory[location]
    }

    pub fn write(&mut self, location: usize, value: T) {
        self.memory[location] = value
    }
}
