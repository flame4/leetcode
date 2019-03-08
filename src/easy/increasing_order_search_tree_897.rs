use crate::Solution;
use crate::treenode::TreeNode;
// TODO 怎么能够避免每一个文件都要use一遍这些公共库的问题呢?
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() { return None; }
        let mut l = root.unwrap().borrow().in_order();
        l.reverse();
        // 为了避免正序过程中的借用问题，可以选择倒着建立树.
        let mut ret = None;
        for i in l {
            let mut tmp = Some(Rc::new(RefCell::new(TreeNode::new(i))));
            tmp.as_mut().unwrap().borrow_mut().right = ret;
            ret = tmp;
        }
        ret
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn increasing_bst() {
        let root = Rc::new(RefCell::new(TreeNode::new(2)));
        root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        root.borrow_mut().left.clone().unwrap().borrow_mut().right =  Some(Rc::new(RefCell::new(TreeNode::new(4))));
        root.borrow_mut().right.clone().unwrap().borrow_mut().right =  Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let mut ret = Solution::increasing_bst(Some(root)).unwrap();
        let mut tmp;
        assert_eq!(ret.borrow().val, 1);
        tmp = ret.borrow_mut().right.clone().unwrap();
        ret = tmp;
        assert_eq!(ret.borrow().val, 4);
        tmp = ret.borrow_mut().right.clone().unwrap();
        ret = tmp;
        assert_eq!(ret.borrow().val, 2);
        tmp = ret.borrow_mut().right.clone().unwrap();
        ret = tmp;
        assert_eq!(ret.borrow().val, 3);
        tmp = ret.borrow_mut().right.clone().unwrap();
        ret = tmp;
        assert_eq!(ret.borrow().val, 7);
    }
}