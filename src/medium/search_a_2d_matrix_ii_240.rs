use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/search-a-2d-matrix-ii/
    /// 在二维数组内查找数字, 二维数组不一定是方形. 并且每一行每一列都是递增的.
    /// 从这个特性可以推出每个点能圈定的最大值范围是这个点左上角的矩形.
    /// 思路1. 从左下角往右上角逼近，直到无法找到一个值. O(m+n)
    /// 思路2, 二分的思路, 先从对角线找起, 但是因为不是矩阵, 复杂度变为
    /// O(m/n*logn + n/(m - n *(m/n)*logm) 不是太漂亮...
    pub fn search_matrix(nums: Vec<Vec<i32>>, target: i32) -> bool {
        true
    }
}