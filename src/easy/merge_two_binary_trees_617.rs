use crate::Solution;
use crate::treenode::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    // 把t2的值加到t1上.
    pub fn merge_trees(t1: Option<Rc<RefCell<TreeNode>>>, t2: Option<Rc<RefCell<TreeNode>>>)
        -> Option<Rc<RefCell<TreeNode>>> {
        // TODO 尝试了半天, 在rust内不容易做到直接递推的赋值节点, 但是代码太复杂了, 感觉可以大大简化.
        // 所以递归的产生节点，然后在本函数体内最后组合得到的结果.
        // 所以只在一个函数内即可.
        // TODO的思路后续再试试.
//        if t1 == None && t2 == None {
//            return None
//        }
//
//        // 计算本结点值.
//        let ret = Rc::new(RefCell::new(TreeNode::new(0)));
//        if t1 != None {
//            ret.borrow_mut().val += t1.unwrap().borrow().val;
//        }
//        if t2 != None {
//            ret.borrow_mut().val += t2.unwrap().borrow().val;
//        }
//
//        // 获得左右节点
//        let t1_left;
//        let t1_right;
//        if let Some(x) = t1 {
//            t1_left = x.borrow().left;
//            t1_right = x.borrow().right;
//        } else {
//            t1_left = None;
//            t1_right = None;
//        }
//
//        let t2_left;
//        let t2_right;
//        if let Some(x) = t2 {
//            t2_left = x.borrow().left;
//            t2_right = x.borrow().right;
//        } else {
//            t2_left = None;
//            t2_right = None;
//        }
//
//        // 递归计算.
//        ret.borrow_mut().left = Solution::merge_trees(t1_left, t2_left);
//        ret.borrow_mut().right = Solution::merge_trees(t1_right, t2_right);
//        Some(ret)
        match (t1, t2) {
            (None, None) => return None,
            (None, Some(n2)) => return Some(n2.clone()),
            (Some(n1),None) => return Some(n1.clone()),
            (Some(n1),Some(n2))=> {
                let v = n1.borrow_mut().val + n2.borrow_mut().val;
                // Important,可以直接clone这里的Option!!!
                // TODO_DONE 但是不能理解, 这样不代表调用了Rc的Clone方法, RC内的引用计数可以正确增加吗?
                // Option实现的Clone要求内部T有Clone的trait bound, 所以没有问题.
                let left_tree = Solution::merge_trees(n1.borrow_mut().left.clone(), n2.borrow_mut().left.clone());
                let right_tree= Solution::merge_trees(n1.borrow_mut().right.clone(), n2.borrow_mut().right.clone());
                return Some(Rc::new(RefCell::new(TreeNode{
                    val : v,
                    left : left_tree,
                    right : right_tree,
                })));
            }
        }
    }


}


#[cfg(test)]
mod tests {
    use crate::Solution;
    use crate::treenode::TreeNode;
    use std::rc::Rc;
    use std::cell::RefCell;


    #[test]
    pub fn merge_2_bin_trees_1() {
        // TODO Use macro to simplify node definition.
        let root1 = Rc::new(RefCell::new(TreeNode::new(1)));
        root1.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        root1.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        root1.borrow_mut().left.clone().unwrap().borrow_mut().left =  Some(Rc::new(RefCell::new(TreeNode::new(5))));

        let root2 = Rc::new(RefCell::new(TreeNode::new(2)));
        root2.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root2.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        root2.borrow_mut().left.clone().unwrap().borrow_mut().right =  Some(Rc::new(RefCell::new(TreeNode::new(4))));
        root2.borrow_mut().right.clone().unwrap().borrow_mut().right =  Some(Rc::new(RefCell::new(TreeNode::new(7))));

        let ret = Solution::merge_trees(Some(root1), Some(root2)).unwrap();
        assert_eq!(ret.borrow().val, 3);
        assert_eq!(ret.borrow().left.clone().unwrap().borrow().val, 4);
        assert_eq!(ret.borrow().right.clone().unwrap().borrow().val, 5);
        assert_eq!(ret.borrow().left.clone().unwrap().borrow().left.clone().unwrap().borrow().val, 5);
        assert_eq!(ret.borrow().left.clone().unwrap().borrow().right.clone().unwrap().borrow().val, 4);
        assert_eq!(ret.borrow().right.clone().unwrap().borrow().right.clone().unwrap().borrow().val, 7);
    }
}