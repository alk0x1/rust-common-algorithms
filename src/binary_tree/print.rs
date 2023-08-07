use super::binary_tree::Node;
use core::fmt::Debug;
use std::borrow::BorrowMut;

pub trait Print<T: Debug> {
  fn search(&self, key: &i32) -> Option<&Node<T>>;
  fn dfs_pre_order_recursive(&self);
  fn dfs_in_order_recursive(&self);
  // fn dfs_post_order_recursive(&self);
  fn dfs_pre_order(&self);
  fn dfs_in_order(&self);
  fn dfs_post_order(&self);
  // fn bfs(&self, key: &i32) -> Option<&Node>;
}

// Implement the Search trait for Node
// impl<T: PartialOrd> Search<T> for Node<T> {
impl Print<i32> for Node<i32>  {
  fn search(&self, key: &i32) -> Option<&Node<i32>> {
    if key == &self.val {
      Some(self)
    } else if key < &self.val {
      match &self.left {
        Some(left) => left.search(key),
        None => None,
      }
    } else {
      match &self.right {
        Some(right) => right.search(key),
        None => None,
      }
    }
  }

  // create new stack frames in each call. May cause stack overflow if the tree is too large
  fn dfs_pre_order_recursive(&self) {
    println!("val: {}" , &self.val);

    match &self.left {
      Some(left) => left.dfs_pre_order_recursive(),
      None => (),
    }
    
    match &self.right {
      Some(right) => right.dfs_pre_order_recursive(),
      None => (),
    }
  }

  // safe way to implement, without stack overflow risk
  fn dfs_pre_order(&self) {
    let mut stack: Vec<&Node<i32>> = vec![];
    stack.push(self);

    while !stack.is_empty() {
      let node = stack.pop().unwrap();
      println!("val: {}", node.val);

      if let Some(right) = &node.right {
        stack.push(right);
      }

      if let Some(left) = &node.left {
        stack.push(left);
      }
    }
  }

  fn dfs_in_order_recursive(&self) {
    if let Some(left) = &self.left {
      left.dfs_in_order();
    }
    
    println!("val: {}", self.val);
    
    if let Some(right) = &self.right {
        right.dfs_in_order();
    }
  }

  fn dfs_in_order(&self) {
    let mut stack: Vec<&Node<i32>> = Vec::new();
    let mut current = Some(self);

    while current.is_some() || !stack.is_empty() {
      while let Some(node) = current {
        stack.push(node);
        current = node.left.as_ref().map(|left| &**left);
      }

      if let Some(node) = stack.pop() {
        println!("val: {}", node.val);
        current = node.right.as_ref().map(|right| &**right);
      }
    }
  }

  // fn dfs_post_order(&self) {
  //   let mut stack: Vec<(&Node<i32>, bool)> = Vec::new();
  //   let mut current = Some(self);

  //   while current.is_some() || !stack.is_empty() {
  //     while let Some(node) = current {
  //       stack.push((node, false));
  //       current = node.left.as_ref().map(|left| &**left);
  //     }

  //     if let Some((node, visited_right)) = stack.pop() {
  //       if !visited_right {
  //         stack.push((node, true));
  //         current = node.right.as_ref().map(|right| &**right);
  //       } else {
  //         println!("val: {}", node.val);
  //         current = None;
  //       }
  //     }
  //   }
  // }
  // fn bfs(&self, key: &i32) -> Option<&Node> {

  // }

  fn dfs_post_order(&self) {
    let mut stack = Vec::new();
    let mut current = Some(self);

    while !stack.is_empty() || current.is_some() {
        while let Some(node) = current {
          stack.push((node, false));
          current = node.left.as_ref().map(|n| &**n);
      }

        if let Some((node, visited_right)) = stack.pop() {
          if !visited_right {
              stack.push((node, true));
              current = node.right.as_ref().map(|n| &**n);
          } else {
              println!("Node value: {:?}", node.val); // Process the node (e.g., print its value)
          }
      }
    }
  }
  // let mut cur = self;
  // let mut stack: Vec<Option<Rc<RefCell<Node<i32>>>>> = vec![];
  // let mut res = vec![];
  
  // while cur.is_some() || !stack.is_empty() {
  //     while let Some(node) = cur {
  //         let right = node.borrow_mut().right.take();
  // cur = right;
  //         res.insert(0, node.borrow().val);
  //         stack.push(Some(node));
  //     }
  //     let node = stack.pop().unwrap().unwrap();
  //     cur = node.borrow_mut().left.take();
  // }
  // res


}





