use std::mem::MaybeUninit;
use std::ops::{Index, IndexMut};

pub struct ComponentPool<T> {
    data: Vec<MaybeUninit<T>>,
    free_list: Vec<usize>,
    len: usize,
}

impl<T> ComponentPool<T> {
    pub fn new(capacity: usize) -> Self {
        let mut data = Vec::with_capacity(capacity);
        data.resize_with(capacity, || MaybeUninit::uninit());

        let mut free_list = Vec::with_capacity(capacity);
        for i in 0..capacity {
            free_list.push(capacity - i - 1);
        }

        Self {
            data,
            free_list,
            len: 0,
        }
    }

    pub fn insert(&mut self, value: T) -> Option<usize> {
        if let Some(index) = self.free_list.pop() {
            unsafe {
                self.data[index].as_mut_ptr().write(value);
            }
            self.len += 1;
            Some(index)
        } else {
            None
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.data.len() || self.free_list.contains(&index) {
            return None;
        }

        self.free_list.push(index);
        self.len -= 1;

        Some(unsafe { std::ptr::read(self.data[index].as_ptr()) })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn capacity(&self) -> usize {
        self.data.len()
    }
}

impl<T> Index<usize> for ComponentPool<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &*self.data[index].as_ptr() }
    }
}

impl<T> IndexMut<usize> for ComponentPool<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut *self.data[index].as_mut_ptr() }
    }
}

impl<T> Drop for ComponentPool<T> {
    fn drop(&mut self) {
        for i in 0..self.data.len() {
            if !self.free_list.contains(&i) {
                unsafe {
                    std::ptr::drop_in_place(self.data[i].as_mut_ptr());
                }
            }
        }
    }
}
