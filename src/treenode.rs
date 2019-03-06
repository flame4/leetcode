use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(dead_code)]
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }


    pub fn pre_order(&self) -> Vec<i32> {
        let mut ret;
        if self.left.is_some() {
            ret = self.left.clone().unwrap().borrow().pre_order();
        } else {
            ret = Vec::new();
        }
        ret.push(self.val);
        if self.right.is_some() {
            ret.extend(self.right.clone().unwrap().borrow().pre_order());
        }
        ret
    }

    pub fn in_order(&self) -> Vec<i32> {
        // TODO
        vec![0]
    }

    pub fn post_order(&self) -> Vec<i32> {
        // TODO
        vec![0]
    }

    pub fn is_leaf(&self) -> bool {
        self.right.is_none() && self.left.is_none()
    }

    pub fn get_leaf_vec(&self) -> Vec<i32> {
        if self.is_leaf() { return vec![self.val]; }
        let mut ret;
        if self.left.is_some() {
            ret = self.left.clone().unwrap().borrow().get_leaf_vec();
        } else {
            ret = Vec::new();
        }
        if self.right.is_some() {
            ret.extend(self.right.clone().unwrap().borrow().pre_order());
        }
        ret
    }
}


#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::cell::RefCell;
    use super::TreeNode;

    #[test]
    pub fn pre_order_test() {
        let root = Rc::new(RefCell::new(TreeNode::new(2)));
        root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        root.borrow_mut().left.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        root.borrow_mut().right.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        assert_eq!(root.borrow().pre_order(), vec![1, 4, 2, 3, 7]);
    }
}


