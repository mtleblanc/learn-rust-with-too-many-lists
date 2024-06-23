// in third.rs

use std::sync::Arc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Arc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn snoc(&self, elem: T) -> Self {
        List {
            head: Some(Arc::new(Node {
                elem: elem,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> Self {
        List {
            head: self.head.as_ref().and_then(|x| x.next.clone()),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|x| &x.elem)
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            wrapped: self.head.as_deref(),
        }
    }
}

pub struct Iter<'a, T> {
    wrapped: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.wrapped.map(|node| {
            self.wrapped = node.next.as_deref();
            &node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node_ref) = head {
            if let Ok(mut node) = Arc::try_unwrap(node_ref) {
                head = node.next.take()
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.snoc(1).snoc(2).snoc(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let list = List::new().snoc(1).snoc(2).snoc(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
}
