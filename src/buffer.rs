pub struct Buffer<T> {
    pub buffer: Vec<T>,
    pub size: i32,
}

impl<T> Buffer<T> {
    pub fn new(buffer: Vec<T>, size: i32) -> Self {
        Buffer { buffer, size }
    }
}
