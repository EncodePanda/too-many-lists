use std::mem;

pub struct List {
  head: Link,
}

pub struct Node {
  elem: i32,
  next: Link,
}

pub enum Link {
  Empty,
  More(Box<Node>),
}

impl List {
  pub fn new() -> Self {
    List { head: Link::Empty }
  }
  pub fn push(&mut self, elem: i32) {
    let newNode = Node {
      elem: elem,
      next: mem::replace(&mut self.head, Link::Empty),
    };
    self.head = Link::More(Box::new(newNode))
  }
}
