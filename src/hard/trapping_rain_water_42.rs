use crate::Solution;
use std::thread::spawn;

impl Solution {
    /// https://leetcode.com/problems/trapping-rain-water/
    /// 我们期望在O(n)的时间内完成计算, 简单的画个图, 可以理解有以下几种情况可以简化描述这个问题.
    /// 1. 在不断向右遍历的过程中, 第一根柱子为x0, 那么遇到的第一个不小于x0的柱子xi, 那么x0到xi之间能产生的
    /// 水位和后面就无关了. 也就是说是一个递增的子问题.
    /// 2. 那么我们只需要考虑左边柱子最大的子问题即可.
    ///
    pub fn trap(height: Vec<i32>) -> i32 {
        spawn()
    }

    /// 这个函数处理右边更大(等)的子问题.
    fn trap_right_higher_sub_question(height: &Vec<i32>, left: usize, right: usize) -> i32 {

    }

    /// 这个函数处理左边更大的子问题.
    fn trap_right_left_sub_question(height: &Vec<i32>, left: usize, right: usize) -> i32 {

    }
}