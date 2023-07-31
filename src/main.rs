mod binary_tree;
use binary_tree::binary_tree::Node;

use crate::binary_tree::search::Search;

fn main() {
    let root = Node::new(1)
        .left(Node::new(2).left(Node::new(4)).right(Node::new(5)))
        .right(Node::new(3).left(Node::new(6)).right(Node::new(7)));

    // println!("{:?}", root);

    root.dfs_pre_order();
}
