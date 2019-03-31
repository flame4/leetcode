use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/longest-increasing-subsequence/
    /// 最长递归子序列问题. 动态规划的解法.
    /// 其中最难处理的一点是类似下面的数列:
    /// 2,3,4,5,6,10,11,7,8,9
    /// 在看到11的时候它还可以是最长序列, 但是我们如果看到7, 我们无法判断后面还跟着什么. 就不能直接在当前的最长子序列上
    /// 操作, 我们必须回看每一个长度的字符串.
    /// 设s[i]是以nums[i]为结尾的最长递归子序列.则每个s[i]可以遍历s[0]到s[i-1]来计算.
    /// O(n^2) 复杂度.
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.is_empty() { return 0; }
        let mut ret = 1;
        let mut s = Vec::new();
        s.resize(nums.len(), 1);
        for i in 1..nums.len() {
            let mut i_max = 1;
            for j in 0..=(i - 1) {
                if nums[j] < nums[i] {
                    i_max = i_max.max(s[j] + 1);
                }
            }
            s[i] = i_max;
            ret = s[i].max(ret);
        }
        ret
    }

    /// 可以理解的一个信息是, 1,2,3 和 1,2,8相比, 我们会选择1,2,3为长度为3的子序列, 以保证后面会有更好的组合.
    /// 那么, 我们在查找的过程中, 可以确认的是, 当前获得的长度为i+1的最优子序列的最后一个数字会比长度为i的更大.
    /// 因为, 长度为i的序列的最优子序列的最后一个数字一定最大是长度为i+1那个的第i个值.
    /// 我们可以用s[i]存储长度为i的数组的最后一个数字.
    /// 用二分查找的方式, 我们可以确认对于一个新的数字, 它会更新哪些数列.
    pub fn length_of_lis2(nums: Vec<i32>) -> i32 {
        let mut s = Vec::new();
        let mut ret = 0;
        s.reserve(nums.len() + 1);
        for i in nums {
            match s.binary_search(&i) {
                Ok(_) => {
                    // nothing, 存在说明这个数字不会被用到了.
                }
                Err(pos) => {
                    if pos >= s.len() {
                        s.push(i)
                    } else {
                        s[pos] = i;
                    }
                    ret = ret.max((pos + 1) as i32);
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
    pub fn length_of_lis_test() {
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
        assert_eq!(Solution::length_of_lis(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 4);
        assert_eq!(Solution::length_of_lis(vec![]), 0);
        assert_eq!(Solution::length_of_lis(vec![1]), 1);
        assert_eq!(Solution::length_of_lis(vec![1, 8, 6, 2, 5, 4, 8, 3, 7, 23, 2, 5, 3, 45, 6, 3, 4, 2, 5]), 6);
        assert_eq!(Solution::length_of_lis(vec![2, 3, 5, 3, 6, 5, 7, 4, 45, 7]), 6);
        assert_eq!(Solution::length_of_lis(vec![2, 3, 5, 4, 76, 7, 4, 5, 6, 7]), 6);
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    }

    #[test]
    pub fn length_of_lis2_test() {
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
        assert_eq!(Solution::length_of_lis2(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 4);
        assert_eq!(Solution::length_of_lis2(vec![]), 0);
        assert_eq!(Solution::length_of_lis2(vec![1]), 1);
        assert_eq!(Solution::length_of_lis2(vec![1, 8, 6, 2, 5, 4, 8, 3, 7, 23, 2, 5, 3, 45, 6, 3, 4, 2, 5]), 6);
        assert_eq!(Solution::length_of_lis2(vec![2, 3, 5, 3, 6, 5, 7, 4, 45, 7]), 6);
        assert_eq!(Solution::length_of_lis2(vec![2, 3, 5, 4, 76, 7, 4, 5, 6, 7]), 6);
        assert_eq!(Solution::length_of_lis2(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    }
}