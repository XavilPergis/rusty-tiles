#![allow(dead_code)]

#[derive(Clone)]
pub struct Queue<T> {
    queue: Vec<T>
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> { Queue { queue: Vec::new() } }
    pub fn enqueue(&mut self, element: T) {
        self.queue.insert(0, element)
    }
    pub fn dequeue(&mut self) -> Option<T> {
        self.queue.pop()
    }
}

unsafe impl<T: Send> Send for Queue<T> {}
unsafe impl<T: Sync> Sync for Queue<T> {}
