use crate::Solution;
use crate::treenode::PackTreeNode;

impl Solution {
    /// https://leetcode.com/problems/path-sum-ii/
    /// 给定一个sum, 找出所有从根到叶子的路径和等于这个值的.
    /// 就DFS吧...
    pub fn path_sum(root: PackTreeNode, sum: i32) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
        if root.is_none() { return ret; }
        let mut path = Vec::new();
        let mut sum_until_now = 0;

        Solution::dfs_path(root, &mut path, &mut sum_until_now, sum, &mut ret);
        ret
    }

    fn dfs_path(node: PackTreeNode, path: &mut Vec<i32>, sum_until_now: &mut i32, sum: i32, ret: &mut Vec<Vec<i32>>) {
        let node = node.unwrap();
        let inner_node = node.borrow();
        path.push(inner_node.val);
        *sum_until_now += inner_node.val;
        if inner_node.is_leaf() && *sum_until_now == sum {
            ret.push(path.clone());
        } else {
            if inner_node.left.is_some() {
                Solution::dfs_path(inner_node.left.clone(), path, sum_until_now, sum, ret);
            }
            if inner_node.right.is_some() {
                Solution::dfs_path(inner_node.right.clone(), path, sum_until_now, sum, ret);
            }
        }
        path.pop();
        *sum_until_now -= inner_node.val;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    use crate::treenode::TreeNode;

    #[test]
    pub fn leaf_similar_test() {
        let v1 = vec![Some(5), Some(4), Some(8), Some(11), None, Some(13), Some(4), Some(7), Some(2), None, None, Some(5), Some(1)];
        let root1 = TreeNode::from_layer_vec(v1);
        assert_eq!(Solution::path_sum(root1, 22), vec![
            vec![5, 4, 11, 2],
            vec![5, 8, 4, 5],
        ]);
    }
}

