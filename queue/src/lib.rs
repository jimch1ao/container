use std::usize;

pub struct Queue<T> {
    pub data: Vec<T>,
    pub cap: usize,
}

impl<T> Queue<T> {
    pub fn new(cap: usize) -> Queue<T> {
        Queue {
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

    pub fn enqueue(&mut self, item: T) -> Result<(), String> {
        if self.data.len() == self.cap {
            return Err("Queue is full".to_string());
        }
        self.data.insert(0, item);
        Ok(())
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.data.len() == 0 {
            return None;
        }
        self.data.pop()
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

pub struct IntoIter<T>(Queue<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.data.len() > 0 {
            self.0.data.pop()
        } else {
            None
        }
    }
}

pub struct Iter<T> {
    queue: Vec<T>,
}
impl<'a, T> Iterator for Iter<&'a T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.queue.len() > 0 {
            self.queue.pop()
        } else {
            None
        }
    }
}

pub struct IterMut<T> {
    queue: Vec<T>,
}
impl<'a, T> Iterator for IterMut<&'a mut T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.queue.len() > 0 {
            self.queue.pop()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue() {
        let mut queue = Queue::new(3);
        let _ = queue.enqueue(1);
        let _ = queue.enqueue(2);
        let _ = queue.enqueue(3);
        assert_eq!(queue.len(), 3);
        assert_eq!(queue.enqueue(4), Err("Queue is full".to_string()));
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), None);
        assert_eq!(queue.is_empty(), true);
    }

    #[test]
    fn into_iter() {
        let mut queue = Queue::new(3);
        let _ = queue.enqueue(1);
        let _ = queue.enqueue(2);
        let _ = queue.enqueue(3);
        let mut iter = queue.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        // assert_eq!(queue.is_empty(), true);
    }

    #[test]
    fn iter() {
        let mut queue = Queue::new(3);
        let _ = queue.enqueue(1);
        let _ = queue.enqueue(2);
        let _ = queue.enqueue(3);
        let mut iter = queue.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));

        assert_eq!(queue.len(), 3);
    }

    #[test]
    fn iter_mut() {
        let mut queue = Queue::new(3);
        let _ = queue.enqueue(1);
        let _ = queue.enqueue(2);
        let _ = queue.enqueue(3);
        let mut iter = queue.iter_mut();

        let item = iter.next().unwrap();
        assert_eq!(item, &mut 1);
        *item = 4;
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(queue.len(), 3);
        assert_eq!(queue.dequeue(), Some(4));
    }
}
