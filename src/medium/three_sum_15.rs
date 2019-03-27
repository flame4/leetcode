use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/3sum/
    /// 3数和问题. 找到数组内所有三个数字加起来为0的组合集合.
    /// 1. 排序数组, 正负分开.
    /// 2. 遍历, 找到第一个不小于0的位置.(如果没有, 游戏结束, 如果这个位置是0, 那么只有一种情况允许, 3个0), 设index=j(第j+1个数字).
    /// 3. 对于一个负数m[i], i<j. 遍历任何 i<k<len(m)-1, k和len(m)-1为两个门限指针, 往中间收缩.
    /// 4. 一种边界情况是处理连续排列的相同数字, 可能会产生相同序列.
    /// 这种情况由两种场景组成, 第一种是顺序遍历的点和上一个相同. 第二个是收缩门限和上一个相同.
    /// 算法复杂度O(n^2)
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
        let mut nums = { nums };
        nums.sort();
        // first not less than 0
        let mut index_fnlt_0 = nums.len();
        for index in 0..nums.len() {
            if nums[index] >= 0 {
                index_fnlt_0 = index;
                break;
            }
        }
        // 2 early stop.
        // all number is less than 0.
        if index_fnlt_0 == nums.len() { return ret; }
        // all number is not less than 0
        if index_fnlt_0 == 0 {
            if index_fnlt_0 + 2 < nums.len()
                && nums[index_fnlt_0 + 1] == 0
                && nums[index_fnlt_0 + 2] == 0 {
                ret.push(vec![0, 0, 0]);
            }
            return ret;
        }

        // end_most 表示在当前index下, 最远需要看多远就可以, 是一种剪枝手段.
        let mut end_most = nums.len() - 1;
        for index in 0..=index_fnlt_0 {
            if (index > 0 && nums[index] == nums[index - 1]) || nums[index] > 0 {
                continue;
            }
            let mut begin = index + 1;
            let mut end = end_most;
            while begin < end {
                if nums[index] * 2 + nums[end_most] > 0 {
                    end_most -= 1;
                    continue;
                }
                if nums[index] + nums[begin] + nums[end] > 0 {
                    end -= 1;
                } else if nums[index] + nums[begin] + nums[end] < 0 {
                    begin += 1;
                } else {
                    // 排除会写入重复的数据情况)
                    if !((begin > index+1 && nums[begin - 1] == nums[begin]) ||
                        (end + 1 < nums.len() && nums[end+1] == nums[end])) {
                        ret.push(vec![nums[index], nums[begin], nums[end]]);
                    }
                    begin += 1;
                    end -= 1;
                }
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    pub fn three_sum_test() {
        let empty: Vec<Vec<i32>> = Vec::new();
        assert_eq!(Solution::three_sum(vec![0, 0, 0]),
                   vec![vec![0, 0, 0]]);
        assert_eq!(Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
                   vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
        assert_eq!(Solution::three_sum(vec![-3, -1, -2, -1, 2, 2, 3]),
                   vec![vec![-2, -1, 3], vec![-1, -1, 2]]);
        assert_eq!(Solution::three_sum(vec![0]),
                   empty.clone());
        assert_eq!(Solution::three_sum(vec![-5, -2, -3, -2, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 4, 3, 4, 5, 6, 7]),
                   vec![vec![-5, -2, 7], vec![-5, -1, 6], vec![-5, 0, 5], vec![-5, 1, 4],
                        vec![-5, 2, 3], vec![-3, -2, 5], vec![-3, -1, 4], vec![-3, 0, 3], vec![-3, 1, 2],
                        vec![-2, -2, 4], vec![-2, -1, 3], vec![-2, 0, 2], vec![-1, 0, 1], vec![0, 0, 0]]);
        assert_eq!(Solution::three_sum(vec![-2, -1, 0, 0, 0, 0, 0]),
                   vec![vec![0, 0, 0]]);
        assert_eq!(Solution::three_sum(vec![-2, 0, 1, 1, 2]),
                   vec![vec![-2, 0, 2], vec![-2, 1, 1]]);
    }
}