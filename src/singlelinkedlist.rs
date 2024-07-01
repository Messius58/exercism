use std::{cell::RefCell, rc::Rc};

#[derive(Clone)]
pub struct Node<T>
where T: Clone {
    item: T,
    next: Option<Rc<RefCell<Node<T>>>>
}

pub struct SimpleLinkedList<T>
where T: Clone {
    peeked: Option<T>,
    current: Option<Rc<RefCell<Node<T>>>>,
    counter: usize
}

impl<T> Node<T>
where T: Clone {
        fn new(val: T) -> Self {
            Self {
            item: val,
            next: None
        }
    }
}

impl<T> SimpleLinkedList<T>
where T: Clone {
    pub fn new() -> Self {
        SimpleLinkedList {
            current: None,
            peeked: None,
            counter: 0
        }
    }

    pub fn is_empty(&self) -> bool {
        self.counter == 0
    }

    pub fn len(&self) -> usize {
        self.counter
    }

    pub fn push(&mut self, _element: T) {
        let mut node = Node::new(_element);
        if self.current.is_some() {
            node.next = Some(Rc::clone(&self.current.as_ref().unwrap()));
        }
        self.current = Some(Rc::new(RefCell::new(node)));
        self.counter += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.current.is_some() {
            let cur = self.current.as_ref().unwrap();
            let node = Some(cur.as_ref().borrow().to_owned().item);
            self.current = if cur.as_ref().borrow().next.is_some() {
                let nxt = Rc::clone(cur.as_ref().borrow().next.as_ref().unwrap());
                Some(nxt)
            }
            else { None };

            self.counter -= 1;

            return node;
        }

        None
    }

    pub fn peek(&mut self) -> Option<&T> {
        if self.current.is_some() {
            self.peeked = Some(self.current.as_ref().unwrap().borrow().item.clone());
            self.peeked.as_ref()
        } else {
            None
        }
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut revll = SimpleLinkedList::new();
        let mut current = self.current;
        while current.is_some() {
            let cur = current.as_ref().unwrap();
            let node = cur.as_ref().borrow().item.clone();
            revll.push(node);
            current = if cur.as_ref().borrow().next.is_some() {
                let nxt = Rc::clone(cur.as_ref().borrow().next.as_ref().unwrap());
                Some(nxt)
            }
            else { None }
        }

        revll
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T>
where T: Clone {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut revll = SimpleLinkedList::new();
        for item in _iter {
            revll.push(item);
        }

        revll
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T>
where T: Clone {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut vector = Vec::<T>::new();
        let mut current = _linked_list.current;
        while current.is_some() {
            let cur = current.as_ref().unwrap();
            let node = cur.as_ref().borrow().item.clone();
            vector.push(node);
            current = if cur.as_ref().borrow().next.is_some() {
                let nxt = Rc::clone(cur.as_ref().borrow().next.as_ref().unwrap());
                Some(nxt)
            }
            else { None }
        }

        vector.reverse();
        vector
    }
}

#[test]
fn new_list_is_empty() {
    let list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    assert_eq!(list.len(), 0, "list's length must be 0");
}
#[test]
fn push_increments_length() {
    let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    list.push(1);
    assert_eq!(list.len(), 1, "list's length must be 1");
    list.push(2);
    assert_eq!(list.len(), 2, "list's length must be 2");
}
#[test]
fn pop_decrements_length() {
    let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    list.push(1);
    list.push(2);
    list.pop();
    assert_eq!(list.len(), 1, "list's length must be 1");
    list.pop();
    assert_eq!(list.len(), 0, "list's length must be 0");
}
#[test]
fn is_empty() {
    let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    assert!(list.is_empty(), "List wasn't empty on creation");
    for inserts in 0..100 {
        for i in 0..inserts {
            list.push(i);
            assert!(
                !list.is_empty(),
                "List was empty after having inserted {i}/{inserts} elements"
            );
        }
        for i in 0..inserts {
            assert!(
                !list.is_empty(),
                "List was empty before removing {i}/{inserts} elements"
            );
            list.pop();
        }
        assert!(
            list.is_empty(),
            "List wasn't empty after having removed {inserts} elements"
        );
    }
}
#[test]
fn pop_returns_head_element_and_removes_it() {
    let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    list.push(1);
    list.push(2);
    assert_eq!(list.pop(), Some(2), "Element must be 2");
    assert_eq!(list.pop(), Some(1), "Element must be 1");
    assert_eq!(list.pop(), None, "No element should be contained in list");
}
#[test]
fn peek_returns_reference_to_head_element_but_does_not_remove_it() {
    let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    assert_eq!(list.peek(), None, "No element should be contained in list");
    list.push(2);
    assert_eq!(list.peek(), Some(&2), "Element must be 2");
    assert_eq!(list.peek(), Some(&2), "Element must be still 2");
    list.push(3);
    assert_eq!(list.peek(), Some(&3), "Head element is now 3");
    assert_eq!(list.pop(), Some(3), "Element must be 3");
    assert_eq!(list.peek(), Some(&2), "Head element is now 2");
    assert_eq!(list.pop(), Some(2), "Element must be 2");
    assert_eq!(list.peek(), None, "No element should be contained in list");
}
#[test]
fn from_slice() {
    let mut array = vec!["1", "2", "3", "4"];
    let mut list: SimpleLinkedList<_> = array.drain(..).collect();
    assert_eq!(list.pop(), Some("4"));
    assert_eq!(list.pop(), Some("3"));
    assert_eq!(list.pop(), Some("2"));
    assert_eq!(list.pop(), Some("1"));
}
#[test]
fn reverse() {
    let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);
    let mut rev_list = list.rev();
    assert_eq!(rev_list.pop(), Some(1));
    assert_eq!(rev_list.pop(), Some(2));
    assert_eq!(rev_list.pop(), Some(3));
    assert_eq!(rev_list.pop(), None);
}
#[test]
fn into_vector() {
    let mut v = Vec::new();
    let mut s = SimpleLinkedList::new();
    for i in 1..4 {
        v.push(i);
        s.push(i);
    }
    let s_as_vec: Vec<i32> = s.into();
    assert_eq!(v, s_as_vec);
}