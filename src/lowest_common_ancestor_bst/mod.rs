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
    let mut cur_root: Rc<RefCell<TreeNode>> = root.unwrap();

    let p: Rc<RefCell<TreeNode>> = p.unwrap();
    let q: Rc<RefCell<TreeNode>> = q.unwrap();

    let p_val = p.borrow().val;
    let q_val = q.borrow().val;

    loop {
        let underlying = cur_root.borrow();
        let mut cur_root_val = underlying.val;

        // go left if both less than current spot
        if p_val < cur_root_val && q_val < cur_root_val {
            cur_root = Rc::clone(underlying.left.as_ref().unwrap());

        // go right if both higher than current spot
        } else if p_val > cur_root_val && q_val > cur_root_val {
            cur_root = Rc::clone(underlying.right.as_ref().unwrap());
        } else {
            return Some(cur_root);
        }
    }
}
