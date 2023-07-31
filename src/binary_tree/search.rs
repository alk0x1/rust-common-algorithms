use super::binary_tree::Node;
use core::fmt::Debug;

pub trait Search<T: Debug> {
  fn search(&self, key: &i32) -> Option<&Node<T>>;
  fn dfs_pre_order(&self) -> ();
  // fn dfs_in_order(&self, key: &i32) -> Option<&Node>;
  // fn dfs_post_order(&self, key: &i32) -> Option<&Node>;
  // fn bfs(&self, key: &i32) -> Option<&Node>;
}

// Implement the Search trait for Node
// impl<T: PartialOrd> Search<T> for Node<T> {
impl Search<i32> for Node<i32>  {
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

  fn dfs_pre_order(&self) {
    println!("val: {}" , &self.val);

    match &self.left {
      Some(left) => left.dfs_pre_order(),
      None => (),
    }
    
    match &self.right {
      Some(right) => right.dfs_pre_order(),
      None => (),
    }

    // if self.left.is_some() {

    // }
  }
  // fn dfs_in_order(&self, key: &i32) -> Option<&Node> {
    
  // }
  // fn dfs_post_order(&self, key: &i32) -> Option<&Node> {
    
  // }
  // fn bfs(&self, key: &i32) -> Option<&Node> {

  // }
}
