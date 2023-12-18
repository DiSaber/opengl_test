pub struct Buffer<T> {
    pub buffer: Vec<T>,
    pub size: usize,
}

impl<T> Buffer<T> {
    pub fn new(buffer: Vec<T>, size: usize) -> Self {
        Buffer { buffer, size }
    }
}
