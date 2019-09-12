use crate::Solution;
use std::rc::Rc;
use std::cell::RefCell;
use crate::treenode::TreeNode;


impl Solution {
    /// https://leetcode.com/problems/invert-binary-tree/
    /// 经典的翻转二叉树, O(n) 复杂度的递归翻转解决方案.
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root_kernel) = root {
            return Some(Solution::invert_tree_internal(root_kernel))
        }
        return None
    }

    /// 我们必须要调用 left 和 right 的 clone 来获取 RC 的一个复制版本. 否则无法用 borrow 出来的 Option 操作.
    /// 为了处理 Option 方便, 声明一个 internal 方法.
    fn invert_tree_internal(root: Rc<RefCell<TreeNode>>) -> Rc<RefCell<TreeNode>> {
        let left_node = root.borrow_mut().left.clone();
        let right_node = root.borrow_mut().right.clone();
        root.borrow_mut().left = Solution::invert_tree(right_node);
        root.borrow_mut().right = Solution::invert_tree(left_node);
        root
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use crate::treenode::TreeNode;
    #[test]
    pub fn invert_tree_test() {
        let root = TreeNode::from_layer_vec(vec![
            Some(4),
            Some(2),
            Some(7),
            Some(1),
            Some(3),
            Some(6),
            Some(9)
        ]);
        assert_eq!(Solution::invert_tree(root).unwrap().borrow().pre_order(),
                   vec![4, 7, 9, 6, 2, 3, 1]);

        let root = TreeNode::from_layer_vec(vec![
            Some(1),
            None,
            Some(2)
        ]);
        assert_eq!(Solution::invert_tree(root).unwrap().borrow().post_order(), vec![2, 1]);
    }
}
