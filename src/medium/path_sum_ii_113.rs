use crate::Solution;
use crate::treenode::PackTreeNode;

impl Solution {
    /// https://leetcode.com/problems/path-sum-ii/
    /// 给定一个sum, 找出所有从根到叶子的路径和等于这个值的.
    /// 就DFS吧...

    pub fn path_sum(root: PackTreeNode, sum: i32) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
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