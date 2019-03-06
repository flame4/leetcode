use crate::Solution;
use crate::treenode::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;


impl Solution {
    pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (root1, root2) {
            (None, None) => return false,
            (None, Some(_)) => return false,
            (Some(_),None) => return false,
            (Some(n1), Some(n2)) => {
                return n1.borrow().get_leaf_vec().eq(&n2.borrow().get_leaf_vec())
            }
        }
    }
}