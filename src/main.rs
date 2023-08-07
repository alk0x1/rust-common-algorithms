mod binary_tree;
mod graphs;
use binary_tree::binary_tree::Node;
use crate::binary_tree::print::Print;

/*
            1
          /  \
        2     3
       / \   / \ 
      4   5 6   7
*/


fn main() {
  let root = Node::new(1)
      .left(Node::new(2).left(Node::new(4)).right(Node::new(5)))
      .right(Node::new(3).left(Node::new(6)).right(Node::new(7)));


  root.dfs_post_order();
}
