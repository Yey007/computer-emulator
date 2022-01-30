pub struct ReadOnlyMemory<T, const N: usize> {
   memory: [T; N]
}

impl<T: Copy, const N: usize> ReadOnlyMemory<T, N> {
    pub fn with_values(values: [T; N]) -> Self {
       ReadOnlyMemory { memory: values }
    }

    pub fn read(&self, location: usize) -> T {
       self.memory[location]
    }
}
