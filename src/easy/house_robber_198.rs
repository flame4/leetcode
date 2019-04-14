use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/house-robber/
    /// 头条面试题...
    /// 找出非连续子序列的最大和, 不接受0值元素.
    /// dp[i] = max(dp[i-1], dp[i-2] + a[i])
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() { return 0; }
        if nums.len() == 1 { return nums[0]; }
        if nums.len() == 2 { return nums[0].max(nums[1]); }
        let mut m = Vec::new();
        m.resize(nums.len(), 0);
        m[0] = nums[0];
        m[1] = nums[1].max(nums[0]);
        for i in 2..nums.len() {
            m[i] = m[i - 1].max(m[i - 2] + nums[i]);
        }
        m.pop().unwrap()
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    pub fn rob_test() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
        assert_eq!(Solution::rob(vec![1,2,23,42,35,3245,243,543,5,3,3,45,34,5,3,45,3,343,5]), 4273);
        assert_eq!(Solution::rob(vec![]), 0);
        assert_eq!(Solution::rob(vec![1]), 1);
        assert_eq!(Solution::rob(vec![1, 2]), 2);
    }
}


