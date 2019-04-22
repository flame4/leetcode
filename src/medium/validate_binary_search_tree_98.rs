use crate::Solution;
use crate::treenode::PackTreeNode;


impl Solution {
    /// https://leetcode.com/problems/validate-binary-search-tree/
    /// 判断一棵树是否是合法的bst, 左边都比节点小, 右边都比节点大.
    /// 递归往上查找即可, 每次需要返回这棵树的最大值和最小值给上面的节点使用.
    pub fn is_valid_bst(root: PackTreeNode) -> bool {
        if root.is_none() { return true; }
        Solution::check_and_get_range(root).is_some()
    }

    /// 功能, 给定一棵树, 首先检查这棵树是否合法, 再求出最后这棵树的最大值和最小值.
    /// 不合法直接返回None.
    fn check_and_get_range(node: PackTreeNode) -> Option<[i32; 2]> {
        if node.is_none() {
            unreachable!()
        }
        let node_context = node.unwrap();
        let mut ret = [node_context.borrow().val, node_context.borrow().val];
        if !node_context.borrow().is_leaf() {
            if node_context.borrow().left.is_some() {
                if let Some(v) = Solution::check_and_get_range(node_context.borrow().left.clone()) {
                    if node_context.borrow().val <= v[1] {
                        return None;
                    } else {
                        ret[0] = v[0];
                    }
                } else {
                    // 子树非法, 则返回.
                    return None;
                }
            }
            if node_context.borrow().right.is_some() {
                if let Some(v) = Solution::check_and_get_range(node_context.borrow().right.clone()) {
                    if node_context.borrow().val >= v[0] {
                        return None;
                    } else {
                        ret[1] = v[1];
                    }
                } else {
                    // 子树非法, 则返回.
                    return None;
                }
            }
        }
        Some(ret)
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;
    use crate::treenode::TreeNode;

    #[test]
    pub fn is_valid_bst_test() {
        let v1 = vec![Some(2), Some(1), Some(3)];
        let root1 = TreeNode::from_layer_vec(v1);
        assert_eq!(Solution::is_valid_bst(root1), true);
        let v2 = vec![Some(5), Some(1), Some(4), None, None, Some(3), Some(6)];
        let root2 = TreeNode::from_layer_vec(v2);
        assert_eq!(false, Solution::is_valid_bst(root2));
    }
}