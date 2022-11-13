use std::mem;

pub struct List {
  head: Node,
}

pub enum Node {
  Empty,
  Next(i32, Box<Node>),
}

impl List {
  pub fn new() -> Self {
    List { head: Node::Empty }
  }

  pub fn push(&mut self, elem: i32) {
    match mem::replace(&mut self.head, Node::Empty) {
      Node::Empty => self.head = Node::Next(elem, Box::new(Node::Empty)),
      tail => {
        let node = Node::Next(elem, Box::new(tail));
        self.head = node
      }
    }
  }

  pub fn pop(&mut self) -> Option<i32> {
    match mem::replace(&mut self.head, Node::Empty) {
      Node::Empty => None,
      Node::Next(elem, node) => {
        self.head = *node;
        Some(elem)
      }
    }
  }
}

mod test {
  #[test]
  fn basics() {
    let mut list = super::List::new();

    // Check empty list behaves right
    assert_eq!(list.pop(), None);
    // Populate list
    list.push(1);
    list.push(2);
    list.push(3);

    // Check normal removal
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(2));

    // Push some more just to make sure nothing's corrupted
    list.push(4);
    list.push(5);

    // Check normal removal
    assert_eq!(list.pop(), Some(5));
    assert_eq!(list.pop(), Some(4));

    // Check exhaustion
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);
  }
}
