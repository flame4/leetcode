use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/jump-game/
    /// 给定一列非负整数, 起始位置在nums[0], 每一个位置表示在这个点上, 你最多可以跳几个offset的距离
    /// 判断是否可以跳到最后一个position.
    /// 首先, 从类型上来说是可以化简为子问题, 考虑递归的做法来实现. 并且应该控制复杂度在O(N)，
    /// 问题是, 如果我们用动态规划来计算, 因为数据内容和长度等长，最坏复杂度已经到达了O(N^2).
    /// 注意到，在某个位置上, 我们不需要知道我们是怎么过来的, 我们只需要知道这个点最远可以到达哪里,
    /// 再知道这个点是否可以达到, 就行了.
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.is_empty() { return true; }
        let mut lastpos = nums[0] as usize;
        for pos in 1..nums.len() {
            if pos > lastpos {
                return false;
            }
            lastpos = lastpos.max(pos + nums[pos] as usize);
        }

        lastpos >= nums.len() - 1
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    pub fn jump_game_test() {
        assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
        assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
        assert_eq!(Solution::can_jump(vec![]), true);
        assert_eq!(Solution::can_jump(vec![0]), true);
    }
}
