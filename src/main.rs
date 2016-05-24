use std::mem;

#[derive(Debug)]
pub struct Node<T> {
    next: Option<Box<Node<T>>>,
    value: T,
}

impl<T> Node<T> {
    pub fn new(value: T, next: Option<Box<Node<T>>>) -> Box<Node<T>> {
        Box::new(Node {
            value: value,
            next: next,
        })
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList { head: None }
    }

    pub fn push_front(&mut self, value: T) {
        match self.head {
            None => self.head = Some(Node::new(value, None)),
            Some(_) => {
                let new_node = Node::new(value, mem::replace(&mut self.head, None));
                self.head = Some(new_node);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match mem::replace(&mut self.head, None) {
            None => None,
            Some(boxed_node) => {
                let node = *boxed_node;
                self.head = node.next;
                Some(node.value)
            }
        }
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    println!("{:?}", list);
    list.pop_front();
    list.pop_front();
    println!("{:?}", list);
}

#[test]
fn test_push_pop_front() {
    let mut list = LinkedList::new();
    list.push_front(1);
    list.push_front(2);
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.pop_front(), Some(1));
}

#[test]
fn test_pop_front_empty_list() {
    let mut list: LinkedList<i32> = LinkedList::new();
    assert_eq!(list.pop_front(), None);
}
