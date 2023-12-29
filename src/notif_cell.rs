pub struct Store<T> {
    value: T,
    listeners: Vec<Box<dyn Fn(T)>>
}

impl<T: Clone> Store<T> {
    pub fn new(inner: T) -> Self {
        Store {
            value: inner,
            listeners: Vec::new()
        }
    }

    pub fn get(&self) -> T {
        self.value.clone()
    }

    pub fn set(&mut self, value: T) {
        self.value = value;
        for listener in self.listeners.iter() {
            listener(self.value.clone())
        }
    }
    
    pub fn on_change<F>(&mut self, listener: F) where F: Fn(T) + 'static {
        self.listeners.push(Box::new(listener));
    }
}