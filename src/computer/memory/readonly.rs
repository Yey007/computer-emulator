pub struct ReadOnlyMemory<T, TAddr, const N: usize> {
    memory: [T; N],
}

impl<T: Copy, TAddr: Into<usize>, const N: usize> ReadOnlyMemory<T, TAddr, N> {
    pub fn with_values(values: [T; N]) -> Self {
        ReadOnlyMemory { memory: values }
    }

    pub fn read(&self, location: TAddr) -> T {
        self.memory[location]
    }
}
