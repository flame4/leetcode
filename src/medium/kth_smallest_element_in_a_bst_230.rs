use crate::Solution;
use crate::treenode::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    /// https://leetcode.com/problems/kth-smallest-element-in-a-bst/
    /// 在一棵二叉查找树内找到第k小的元素.
    /// 思路自然是中序遍历的过程中记录.
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut ret = 0;
        Solution::recursive_kth_smallest_search(root.unwrap(), k, &mut 0, &mut ret);
        ret
    }

    /// Given Root is always valid.
    fn recursive_kth_smallest_search(root: Rc<RefCell<TreeNode>>, k: i32, search_cnt: &mut i32, ret: &mut i32) {
        if *search_cnt >= k {
            return
        }
        if root.borrow().left.is_some() {
            Solution::recursive_kth_smallest_search(root.borrow().left.clone().unwrap(), k, search_cnt, ret);
        }
        if *search_cnt >= k {
            return
        }
        *search_cnt += 1;
        if *search_cnt == k {
            *ret = root.borrow().val;
            return
        }
        if root.borrow().right.is_some() {
            Solution::recursive_kth_smallest_search(root.borrow().right.clone().unwrap(), k, search_cnt, ret);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use crate::treenode::TreeNode;
    #[test]
    pub fn kth_smallest_test() {
        let root = TreeNode::from_layer_vec(vec![
            Some(5),
            Some(3),
            Some(6),
            Some(2),
            Some(4),
            None,
            None,
            Some(1)
        ]);
        assert_eq!(Solution::kth_smallest(root, 3), 3);
        let root = TreeNode::from_layer_vec(vec![
            Some(1),
            None,
            Some(2)
        ]);
        assert_eq!(Solution::kth_smallest(root, 2), 2);
    }
}

