use std::cell::RefCell;
use std::rc::Rc;

use self::tree_node::TreeNode;
mod tree_node {
    use std::{cell::RefCell, rc::Rc};

    // Definition for a binary tree node.
    #[derive(Debug, PartialEq, Eq)]
    pub struct TreeNode {
        pub val: i32,
        pub left: Option<Rc<RefCell<TreeNode>>>,
        pub right: Option<Rc<RefCell<TreeNode>>>,
    }

    impl TreeNode {
        #[inline]
        pub fn new(val: i32) -> Self {
            TreeNode {
                val,
                left: None,
                right: None,
            }
        }
    }
}

pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let p_val = p.unwrap().borrow().val;
    let q_val = q.unwrap().borrow().val;
    let mut cur_root = root.unwrap();

    loop {
        let cur_node = cur_root.borrow();
        let cur_root_val = cur_node.val;

        // go left if both less than current spot
        if p_val < cur_root_val && q_val < cur_root_val {
            let new_root = Rc::clone(cur_node.left.as_ref().unwrap());
            drop(cur_node);
            cur_root = new_root;
        // go right if both higher than current spot
        } else if p_val > cur_root_val && q_val > cur_root_val {
            let new_root = Rc::clone(cur_node.right.as_ref().unwrap());
            drop(cur_node);
            cur_root = new_root;
        } else {
            drop(cur_node);
            return Some(cur_root);
        }
    }
}
