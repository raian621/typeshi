#![allow(dead_code)] // TODO: Remove once we use this somewhere
use std::{cell::RefCell, rc::Rc};

type QueueNodeRef<T> = Rc<RefCell<QueueNode<T>>>;

pub struct Queue<T> {
    head: Option<QueueNodeRef<T>>,
    tail: Option<QueueNodeRef<T>>,
    size: usize,
}

struct QueueNode<T> {
    pub value: T,
    next: Option<QueueNodeRef<T>>,
}

impl<T: Copy> Queue<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn append(&mut self, value: T) {
        let node = Some(Rc::new(RefCell::new(QueueNode { value, next: None })));
        self.size += 1;
        let old_tail = &self.tail;
        if let Some(old_tail) = old_tail {
            old_tail.borrow_mut().next = node.clone();
            self.tail = node;
        } else {
            self.head = node.clone();
            self.tail = node.clone();
        }
    }

    pub fn prepend(&mut self, value: T) {
        let node = Rc::new(RefCell::new(QueueNode { value, next: None }));
        self.size += 1;
        if self.head.is_some() {
            // Set node.next to self.head, move existing None object into self.head to avoid
            // additional clones.
            std::mem::swap(&mut node.borrow_mut().next, &mut self.head);
            self.head = Some(node);
        } else {
            self.head = Some(node.clone());
            self.tail = Some(node.clone());
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let old_head = self.head.clone();
        if let Some(old_head) = old_head {
            self.head = old_head.borrow().next.clone();
            if self.head.is_none() {
                self.tail = None;
            }
            old_head.borrow_mut().next = None;
            self.size -= 1;
            Some(old_head.borrow().value)
        } else {
            None
        }
    }
}

impl<T: Copy> Into<Vec<T>> for Queue<T> {
    fn into(self) -> Vec<T> {
        let mut curr = self.head.clone();
        let mut result = vec![];
        result.reserve_exact(self.size);

        while curr.is_some() {
            let curr_node = curr.unwrap().clone();
            result.push(curr_node.borrow().value);
            curr = curr_node.borrow().next.clone()
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append() {
        let expected = vec![1, 2, 3, 4, 5];
        let mut queue = Queue::new();
        expected.iter().for_each(|x| queue.append(*x));
        assert_eq!(expected, Into::<Vec<i32>>::into(queue));
    }

    #[test]
    fn test_prepend() {
        let reverse_expected = vec![1, 2, 3, 4, 5];
        let mut queue = Queue::new();
        reverse_expected.iter().for_each(|x| queue.prepend(*x));
        assert_eq!(
            reverse_expected.into_iter().rev().collect::<Vec<i32>>(),
            Into::<Vec<i32>>::into(queue)
        );
    }

    #[test]
    fn test_pop_front() {
        let expected = vec![1, 2, 3, 4, 5];
        let mut queue = Queue::new();
        expected.iter().for_each(|x| queue.append(*x));
        let mut result = vec![];
        while let Some(x) = queue.pop_front() {
            result.push(x);
        }
        assert_eq!(expected, result);
    }

    #[test]
    fn test_append_prepend_and_pop_front() {
        let input = vec![1, 2, 3, 4, 5];
        let mut queue = Queue::new();
        input.iter().for_each(|x| queue.append(*x));
        input.iter().for_each(|x| queue.prepend(*x));
        let expected = input
            .iter()
            .cloned()
            .rev()
            .chain(input.iter().cloned())
            .collect::<Vec<i32>>();
        let mut result = vec![];
        while let Some(x) = queue.pop_front() {
            result.push(x);
        }
        assert_eq!(expected, result);
    }
}
