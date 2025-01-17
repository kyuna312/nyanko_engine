use std::collections::VecDeque;

pub struct ObjectPool<T> {
    active: Vec<T>,
    available: VecDeque<T>,
    max_size: usize,
}

impl<T: Default> ObjectPool<T> {
    pub fn new(initial_size: usize, max_size: usize) -> Self {
        let mut available = VecDeque::with_capacity(initial_size);
        for _ in 0..initial_size {
            available.push_back(T::default());
        }

        Self {
            active: Vec::new(),
            available,
            max_size,
        }
    }

    pub fn spawn(&mut self) -> Option<T> {
        self.available.pop_front().or_else(|| {
            if self.active.len() < self.max_size {
                Some(T::default())
            } else {
                None
            }
        })
    }

    pub fn recycle(&mut self, object: T) {
        self.available.push_back(object);
    }
}
