use std::rc::Rc;
use std::cell::RefCell;
use std::collections::vec_deque::VecDeque;


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

    /// 层次遍历法构造树, 和leetcode给出的方式一样.
    /// From https://github.com/Aloxaf/leetcode_prelude/blob/master/leetcode_prelude/src/btree.rs, thx :-)
    pub fn from_layer_vec(input: Vec<Option<i32>>) -> Option<Rc<RefCell<Self>>> {
        if input.is_empty() { return None; }
        // Queue内的节点装的是所有准备好 **给他们的儿子赋值** 的节点
        let mut queue = VecDeque::new();
        let root = Some(Rc::new(RefCell::new(TreeNode::new(input[0].unwrap()))));
        queue.push_back(root.clone());
        for i in input[1..].chunks(2) {
            let node = queue.pop_front().unwrap().unwrap();
            if let Some(val) = i[0] {
                node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                queue.push_back(node.borrow().left.clone());
            }
            if i.len() < 2 { break; }
            if let Some(val) = i[1] {
                node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                queue.push_back(node.borrow().right.clone());
            }
        }
        root
    }

    pub fn in_order(&self) -> Vec<i32> {
        let mut ret;
        if self.left.is_some() {
            ret = self.left.clone().unwrap().borrow().in_order();
        } else {
            ret = Vec::new();
        }
        ret.push(self.val);
        if self.right.is_some() {
            ret.extend(self.right.clone().unwrap().borrow().in_order());
        }
        ret
    }

    pub fn pre_order(&self) -> Vec<i32> {
        let mut ret = vec![self.val];
        if self.left.is_some() {
            ret.extend(self.left.clone().unwrap().borrow().pre_order())
        }
        if self.right.is_some() {
            ret.extend(self.right.clone().unwrap().borrow().pre_order())
        }
        ret
    }

    pub fn post_order(&self) -> Vec<i32> {
        let mut ret;
        match (self.left.clone(), self.right.clone()) {
            (None, None) => ret = Vec::new(),
            (Some(l), None) => {
                ret = l.borrow().post_order();
            }
            (None, Some(r)) => {
                ret = r.borrow().post_order();
            }
            (Some(l), Some(r)) => {
                ret = l.borrow().post_order();
                ret.extend(r.borrow().post_order());
            }
        }
        ret.push(self.val);
        ret
    }

    pub fn is_leaf(&self) -> bool {
        self.right.is_none() && self.left.is_none()
    }

    /// Get The Leaf Node Of This Tree.
    /// Definition reference: easy/leaf_similar_tree_872.rs
    pub fn get_leaf_vec(&self) -> Vec<i32> {
        if self.is_leaf() { return vec![self.val]; }
        let mut ret;
        if self.left.is_some() {
            ret = self.left.clone().unwrap().borrow().get_leaf_vec();
        } else {
            ret = Vec::new();
        }
        if self.right.is_some() {
            ret.extend(self.right.clone().unwrap().borrow().get_leaf_vec());
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
    pub fn in_order_test() {
        let root = Rc::new(RefCell::new(TreeNode::new(2)));
        root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        root.borrow_mut().left.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        root.borrow_mut().right.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        assert_eq!(root.borrow().in_order(), vec![1, 4, 2, 3, 7]);
    }

    #[test]
    pub fn pre_order_test() {
        let root = Rc::new(RefCell::new(TreeNode::new(2)));
        root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        root.borrow_mut().left.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        root.borrow_mut().right.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        assert_eq!(root.borrow().pre_order(), vec![2, 1, 4, 3, 7]);
    }

    #[test]
    pub fn post_order_test() {
        let root = Rc::new(RefCell::new(TreeNode::new(2)));
        root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        root.borrow_mut().left.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        root.borrow_mut().right.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        assert_eq!(root.borrow().post_order(), vec![4, 1, 7, 3, 2]);
    }

    #[test]
    pub fn from_layer_vec_test() {
        let v = vec![Some(2), Some(1), Some(3), None, Some(4), None, Some(7)];
        let tree = TreeNode::from_layer_vec(v);
        assert_eq!(tree.unwrap().borrow().pre_order(), vec![2, 1, 4, 3, 7]);
    }
}


