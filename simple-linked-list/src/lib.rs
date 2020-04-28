use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            head: None,
            size: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, element: T) {
        let node = Node {
            data: element,
            next: self.head.take(),
        };
        self.head = Some(Box::new(node));
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(node) = self.head.take() {
            self.head = node.next;
            self.size -= 1;
            Some(node.data)
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            Some(node) => Some(&node.data),
            None => None,
        }
    }

    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut v = Vec::with_capacity(self.size);
        while let Some(item) = self.pop() {
            v.push(item);
        }
        SimpleLinkedList::from_iter(v.drain(..))
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            current: &self.head,
        }
    }
}

impl<'a, T: 'a> IntoIterator for &'a SimpleLinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for item in iter {
            list.push(item);
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut v = Vec::with_capacity(self.size);
        while let Some(item) = self.pop() {
            v.insert(0, item);
        }
        v
    }
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct Iter<'a, T: 'a> {
    current: &'a Option<Box<Node<T>>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        match &self.current {
            &Some(node) => {
                let data = &node.data;
                self.current = &node.next;
                Some(data)
            }
            &None => None,
        }
    }
}
