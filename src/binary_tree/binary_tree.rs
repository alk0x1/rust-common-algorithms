#[derive(Debug)] // Add the Debug trait to allow easy printing for demonstration purposes.
pub struct Node<T> {
  pub val: T,
  pub left: Option<Box<Node<T>>>,
  pub right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
  pub fn new(val: T) -> Self {
    Node {
      val,
      left: None,
      right: None,
    }
  }

  pub fn left(mut self, node: Node<T>) -> Self {
    self.left = Some(Box::new(node));
    self
  }

  pub fn right(mut self, node: Node<T>) -> Self {
    self.right = Some(Box::new(node));
    self
  }
}
