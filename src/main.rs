use std::borrow::Borrow;
use std::fmt::Display;
use tool::empty::IsEmpty;
use std::default::Default;

//Linked list style LIFO stack

type NodeOption<T> = Option<Box<Node<T>>>;

struct Node<T> {
    _node: NodeOption<T>,
    _data: T
}

struct LinkedStack<T> {
    _top: NodeOption<T>,
    _count: usize
}

struct Iter<'a, T> {
    _reference: & 'a NodeOption<T>
}

impl<'a, T> Iterator for Iter<'a, T>
    where T: Display + Clone

{
    type Item = T;

    fn next(& mut self) -> Option<T> {
        match self._reference {
            Some(node) => {
                let val = node._data.clone();
                self._reference = &node._node;
                Some(val)
            },
            None => None
        }
    }
}

impl<T> LinkedStack<T>
    where T: Display + Clone
{
    pub fn new() -> Self {
        LinkedStack {
            _top: None,
            _count: 0
        }
    }

    pub fn push(&mut self, item: T) {

        let new_node = Box::new(
            Node {
                _node: self._top.take(),
                _data: item
            }
        );

        self._top = Some(new_node);

        self._count += 1;

    }

    pub fn pop(& mut self) -> Result<T, &str> {
        match self._top.take() {
            Some(head) => {
                let data = head._data;
                self._top = head._node;
                self._count -= 1;
                Ok(data)
            },
            None => Err("Cannot pop empty stack")
        }

    }

    pub fn len(&self) -> usize {
        self._count
    }

    pub fn print(&self) {
        let mut re = &self._top;
        loop {
            match re {
                Some(t) => {
                    println!("{}", t._data);
                    re = &t._node;
                }
                None => break
            }
        }
    }

    fn recursive_clone(&self, re: &NodeOption<T>, mut clne: Self) -> Self {
        match re {
            Some(boxed_node) => {
                let mut cloned = self.recursive_clone(&boxed_node._node, clne);
                cloned.push(boxed_node._data.clone());
                cloned
            }
            None => clne
        }
    }

}

impl<T> IsEmpty for LinkedStack<T>
    where T: Display + Clone
{
    fn is_empty(&self) -> bool {
        self._count == 0
    }
}

impl<T> Default for LinkedStack<T>
    where T: Display + Clone
{
    fn default() -> Self {
        LinkedStack::new()
    }
}

impl<T> Clone for LinkedStack<T>
    where T: Display + Clone
{
    fn clone(&self) -> Self {
        self.recursive_clone(&self._top, Self::new())
    }
}

fn main() {
    println!("Hello, world!");

    let mut ll = LinkedStack::new();

    ll.push(Box::new(33));
    ll.push(Box::new(2));

    ll.push(Box::new(4));
    ll.push(Box::new(5));

    ll.print();

    let cl = ll.clone();

    cl.print();




}
