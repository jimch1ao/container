#[derive(Debug)]
pub struct Deque<T> {
    pub data: Vec<T>,
    pub cap: usize,
}

impl<T> Deque<T> {
    pub fn new(cap: usize) -> Self {
        Self {
            data: Vec::with_capacity(cap),
            cap: cap,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.data = Vec::with_capacity(self.cap);
    }

    pub fn is_full(&self) -> bool {
        self.data.len() == self.cap
    }

    pub fn add_front(&mut self, item: T) -> Result<(), String> {
        if self.is_full() {
            return Err("Deque is full".to_string());
        }
        self.data.push(item);
        Ok(())
    }

    pub fn add_rear(&mut self, item: T) -> Result<(), String> {
        if self.is_full() {
            return Err("Deque is full".to_string());
        }
        self.data.insert(0, item);
        Ok(())
    }

    pub fn remove_front(&mut self) -> Option<T> {
        if self.data.len() > 0 {
            return self.data.pop();
        }
        None
    }

    pub fn remove_rear(&mut self) -> Option<T> {
        if self.data.len() > 0 {
            return Some(self.data.remove(0));
        }
        None
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<&T> {
        let mut iter = Iter {
            queue: Vec::with_capacity(self.data.len()),
        };
        for item in self.data.iter() {
            iter.queue.push(item)
        }
        iter
    }

    pub fn iter_mut(&mut self) -> IterMut<&mut T> {
        let mut iter = IterMut {
            queue: Vec::with_capacity(self.data.len()),
        };
        for item in self.data.iter_mut() {
            iter.queue.push(item)
        }
        iter
    }
}

pub struct IntoIter<T>(Deque<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.data.len() > 0 {
            return self.0.data.pop();
        }
        None
    }
}

pub struct Iter<T> {
    queue: Vec<T>,
}
impl<'a, T> Iterator for Iter<&'a T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.queue.len() > 0 {
            return self.queue.pop();
        }
        None
    }
}

pub struct IterMut<T> {
    queue: Vec<T>,
}
impl<'a, T> Iterator for IterMut<&'a mut T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.queue.len() > 0 {
            return self.queue.pop();
        }
        None
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn Deque() {
        let mut deque = Deque::new(3);
        let _ = deque.add_rear(2);
        let _ = deque.add_rear(3);
        let _ = deque.add_front(1);

        let r = deque.add_front(1);
        assert_eq!(r, Err("Deque is full".to_string()));
        assert_eq!(deque.len(), 3);
        assert_eq!(deque.remove_rear(), Some(3));
        assert_eq!(deque.remove_front(), Some(1));
        assert_eq!(deque.remove_front(), Some(2));
        assert_eq!(deque.remove_front(), None);
        assert_eq!(deque.is_empty(), true);
    }

    #[test]
    fn into_iter(){
        let mut deque = Deque::new(3);
        let _ = deque.add_rear(2);
        let _ = deque.add_rear(3);
        let _ = deque.add_front(1);

        let mut iter = deque.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }
}
