use crate::Solution;
use crate::treenode::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;


impl Solution {
    pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (root1, root2) {
            (None, None) => return false,
            (None, Some(_)) => return false,
            (Some(_), None) => return false,
            (Some(n1), Some(n2)) => {
                return n1.borrow().get_leaf_vec().eq(&n2.borrow().get_leaf_vec());
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;
    use crate::treenode::TreeNode;

    #[test]
    pub fn leaf_similar_test() {
        let v1 = vec![Some(3), Some(5), Some(1), Some(6), Some(2), Some(9), Some(8), None, None, Some(7), Some(4)];
        let root1 = TreeNode::from_layer_vec(v1);
        let v2 = vec![Some(3), Some(5), Some(1), Some(6), Some(7), Some(4), Some(2), None, None, None, None, None, None, Some(9), Some(8)];
        let root2 = TreeNode::from_layer_vec(v2);
        assert_eq!(true, Solution::leaf_similar(root1, root2));
    }
}