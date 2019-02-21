use super::Solution;

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

    // TODO: implement a function to show tree.
    pub fn show() {

    }
}

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    // 把t2的值加到t1上.
    pub fn merge_trees(t1: Option<Rc<RefCell<TreeNode>>>, t2: Option<Rc<RefCell<TreeNode>>>)
        -> Option<Rc<RefCell<TreeNode>>> {
        Solution::merge_node(t1, t2);

    }

    fn merge_node(t1: Option<Rc<RefCell<TreeNode>>>, t2: Option<Rc<RefCell<TreeNode>>>) {
        if t2 == None {
            return
        }
        let mut t2_node = t2.unwrap().borrow_mut();

        let mut t1_rc;
        if t1 == None {
            t1_rc = Rc::new(RefCell::new(TreeNode::new(0)));
        } else {
            t1_rc = t1.unwrap();
        }
        let mut t1_node = t1_rc.borrow_mut();

        t1_node.val += t2_node.val;
        Solution::merge_node(t1_node.left, t2_node.left);
        Solution::merge_node(t1_node.right, t2_node.right);
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;
    use super::TreeNode;
    use std::rc::Rc;
    use std::cell::RefCell;


    #[test]
    pub fn merge_2_bin_trees_1() {
        // TODO Use macro to simplify node definition.
        let mut root1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root1.unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        root1.unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        root1.unwrap().borrow_mut().left.unwrap().borrow_mut().left =  Some(Rc::new(RefCell::new(TreeNode::new(5))));

        let mut root2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        root1.unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root1.unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        root1.unwrap().borrow_mut().left.unwrap().borrow_mut().right =  Some(Rc::new(RefCell::new(TreeNode::new(4))));
        root1.unwrap().borrow_mut().right.unwrap().borrow_mut().right =  Some(Rc::new(RefCell::new(TreeNode::new(7))));

        let ret = Solution::merge_trees(root1, root2);
        assert_eq!(ret.unwrap().borrow().val, 3);
        assert_eq!(ret.unwrap().borrow().left.unwrap().borrow().val, 4);
        assert_eq!(ret.unwrap().borrow().right.unwrap().borrow().val, 5);
        assert_eq!(ret.unwrap().borrow().left.unwrap().borrow().left.unwrap().borrow().val, 5);
        assert_eq!(ret.unwrap().borrow().left.unwrap().borrow().right.unwrap().borrow().val, 4);
        assert_eq!(ret.unwrap().borrow().right.unwrap().borrow().right.unwrap().borrow().val, 7);
    }
}