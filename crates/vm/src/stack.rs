/// Stack implementation for the VM, has some convenience methods.
#[derive(Debug, Clone)]
pub struct Stack<T> {
    inner: Vec<T>,   
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            inner: Vec::new(),
        }
    }

    pub fn from(item: T) -> Self {
        Self {
            inner: vec![item],
        }
    }

    pub fn inner_vec(&self) -> &Vec<T> {
        &self.inner
    }

    pub fn push(&mut self, value: T) {
        self.inner.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop()
    }

    pub fn last_mut(&mut self) -> Option<&mut T> {
        self.inner.last_mut()
    }

    pub fn last(&self) -> Option<&T> {
        self.inner.last()
    }

    pub fn pop_many_into(&mut self, count: usize, target: &mut Vec<T>) {
        for _ in 0..count {
            if let Some(value) = self.pop() {
                target.push(value);
            } else {
                break;
            }
        }
    }

    pub fn pop_many(&mut self, count: usize) -> Vec<T> {
        let mut result = Vec::with_capacity(count);
        self.pop_many_into(count, &mut result);
        result
    }

    pub fn first(&self) -> Option<&T> {
        self.inner.first()
    }

    pub fn first_mut(&mut self) -> Option<&mut T> {
        self.inner.first_mut()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}
