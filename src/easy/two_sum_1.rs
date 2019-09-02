use crate::Solution;
use std::collections::HashMap;


impl Solution {
    /// https://leetcode.com/problems/two-sum/
    /// 二数和问题, 基于 hash 来做计数, O(n) 的时间复杂度和 o(n) 空间复杂度.
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut m = HashMap::new();
        for (index, val) in nums.iter().enumerate() {
            match m.get(&(target - val)) {
                Some(opposite_index) => {
                    return vec![*opposite_index as i32, index as i32]
                },
                None => {
                    m.insert(val, index);
                }
            }
        }
        vec![]
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    pub fn two_sum_test() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
}
