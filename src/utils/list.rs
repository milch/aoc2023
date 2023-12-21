use std::rc::Rc;

#[derive(Debug)]
enum Node<T> {
    Value(T, Rc<Node<T>>),
    Empty,
}

impl<T> PartialEq for Node<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Value(l, l_tail), Self::Value(r, r_tail)) => l == r && l_tail == r_tail,
            (Self::Empty, Self::Empty) => true,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}
impl<T> Eq for Node<T> where T: Eq {}

#[derive(Debug)]
pub struct List<T> {
    head: Rc<Node<T>>,
    len: usize,
}

pub struct ListIterator<'a, T> {
    current_node: &'a Node<T>,
}

impl<T> List<T> {
    pub fn iter(&self) -> ListIterator<T> {
        ListIterator {
            current_node: &self.head,
        }
    }

    pub fn new() -> Self {
        List {
            head: Rc::new(Node::Empty),
            len: 0,
        }
    }

    pub fn prepend(&self, value: T) -> Self {
        List {
            head: Rc::new(Node::Value(value, self.head.clone())),
            len: self.len + 1,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn front(&self) -> Option<&T> {
        match self.head.as_ref() {
            Node::Empty => None,
            Node::Value(val, _) => Some(val),
        }
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T> Iterator for ListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_node {
            Node::Value(val, next) => {
                self.current_node = next;
                Some(val)
            }
            Node::Empty => None,
        }
    }
}

impl<T> PartialEq for List<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.head == other.head
    }
}

impl<T> Eq for List<T> where T: Eq {}
