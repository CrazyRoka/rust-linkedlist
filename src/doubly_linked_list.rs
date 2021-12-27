use std::{cell::RefCell, rc::Rc};

struct ListNode<T> {
    item: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> ListNode<T> {
    fn new(item: T) -> Self {
        Self {
            item,
            next: None,
            prev: None,
        }
    }
}

type Link<T> = Option<Rc<RefCell<ListNode<T>>>>;

#[derive(Default)]
pub struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    size: usize,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn push_back(&mut self, item: T) {
        let node = Rc::new(RefCell::new(ListNode::new(item)));
        if let Some(prev_tail) = self.tail.take() {
            prev_tail.borrow_mut().next = Some(Rc::clone(&node));
            node.borrow_mut().prev = Some(prev_tail);
            self.tail = Some(node);
            self.size += 1;
        } else {
            self.head = Some(Rc::clone(&node));
            self.tail = Some(node);
            self.size = 1;
        }
    }

    pub fn push_front(&mut self, item: T) {
        let node = Rc::new(RefCell::new(ListNode::new(item)));
        if let Some(prev_head) = self.head.take() {
            prev_head.borrow_mut().prev = Some(Rc::clone(&node));
            node.borrow_mut().next = Some(prev_head);
            self.head = Some(node);
            self.size += 1;
        } else {
            self.head = Some(Rc::clone(&node));
            self.tail = Some(node);
            self.size = 1;
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|prev_tail| {
            self.size -= 1;
            match prev_tail.borrow_mut().prev.take() {
                Some(node) => {
                    node.borrow_mut().next = None;
                    self.tail = Some(node);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(prev_tail).ok().unwrap().into_inner().item
        })
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|prev_head| {
            self.size -= 1;
            match prev_head.borrow_mut().next.take() {
                Some(node) => {
                    node.borrow_mut().prev = None;
                    self.head = Some(node);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(prev_head).ok().unwrap().into_inner().item
        })
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }
}

impl<T> IntoIterator for DoublyLinkedList<T> {
    type Item = <ListIterator<T> as Iterator>::Item;

    type IntoIter = ListIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        while let Some(node) = self.head.take() {
            let _ = node.borrow_mut().prev.take();
            self.head = node.borrow_mut().next.take();
        }
        self.tail.take();
    }
}

pub struct ListIterator<T> {
    list: DoublyLinkedList<T>,
}

impl<T> ListIterator<T> {
    fn new(list: DoublyLinkedList<T>) -> Self {
        Self { list }
    }
}

impl<T> Iterator for ListIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }
}

impl<T> DoubleEndedIterator for ListIterator<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.list.pop_back()
    }
}

#[cfg(test)]
mod tests {
    use super::DoublyLinkedList;

    #[test]
    fn it_works() {
        let mut list = DoublyLinkedList::new();

        list.push_back(3);
        list.push_back(4);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.len(), 1);

        list.push_front(5);
        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), None);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn can_push_back() {
        let mut list = DoublyLinkedList::new();
        assert_eq!(list.pop_back(), None);

        list.push_back(3);
        list.push_back(4);
        list.push_back(5);
        assert_eq!(list.pop_back(), Some(5));

        list.push_back(6);
        list.push_back(7);
        assert_eq!(list.pop_back(), Some(7));
        assert_eq!(list.pop_back(), Some(6));
        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.pop_back(), Some(3));

        list.push_back(2);
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn it_drops_correctly() {
        let mut list = DoublyLinkedList::new();
        for i in 0..3 {
            list.push_back(i);
        }

        drop(list);
    }

    #[test]
    fn can_iterate_forward() {
        let mut list = DoublyLinkedList::new();
        for i in 0..10 {
            list.push_front(i);
        }

        assert!(Iterator::eq(list.into_iter(), (0..10).rev()));
    }

    #[test]
    fn can_iterate_back() {
        let mut list = DoublyLinkedList::new();
        for i in 0..10 {
            list.push_front(i);
        }

        assert!(Iterator::eq(list.into_iter().rev(), 0..10));
    }
}
