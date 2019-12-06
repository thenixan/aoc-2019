use std::cell::RefCell;
use std::collections::VecDeque;
use std::iter::{IntoIterator, Iterator};
use std::ops::AddAssign;
use std::rc::Rc;

pub struct Node<T> {
    elem: T,
    children: Vec<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(t: T) -> Self {
        Node {
            elem: t,
            children: vec![],
        }
    }

    pub fn value(&self) -> T {
        self.elem
    }
}

impl<T: Eq> AddAssign for Node<T> {
    fn add_assign(&mut self, other: Self) {
        if other.elem == self.elem {
            self.children.append(&mut other.children);
        } else {
            self.children.push(Rc::new(RefCell::new(other)));
        }
    }
}

struct NodeIterator<T> {
    stack: VecDeque<Rc<RefCell<Node<T>>>>,
}

impl<T> Iterator for NodeIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if let Some(item) = self.stack.pop_front() {
            let result = item.borrow().elem;
            let mut new = item
                .borrow()
                .children
                .iter()
                .map(|i| Rc::clone(i))
                .collect::<VecDeque<_>>();
            self.stack.append(&mut new);
            Some(result)
        } else {
            None
        }
    }
}

impl<T> IntoIterator for Node<T> {
    type Item = T;
    type IntoIter = NodeIterator<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut result = NodeIterator {
            stack: VecDeque::new(),
        };
        result.stack.push_back(Rc::new(RefCell::new(self)));
        result
    }
}
