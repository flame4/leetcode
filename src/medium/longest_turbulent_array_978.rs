use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/longest-turbulent-subarray/
    /// 题目要求给出两种对偶情况下的最优解，可以先考虑其中一种，考虑清楚了再拓展到对偶情况.
    /// 先考虑 a[k] > a[k+1] when k is odd, else a[k] < a[k+1]; (condition 1)
    /// 然后对偶情况扩充到  a[k] > a[k+1] when k is even, else a[k] < a[k+1] (condition 2).
    /// 对于condition 1. 可以用动态规划. 定义m[i]是以i为结尾的子串的最大长度.
    /// m[i+1]  = 1 if (i is odd and a[i] <= a[i+1]) or (i is even and a[i] >= a[i+1])
    ///         = m[i+1] if (i is odd and a[i] > a[i+1]) or (i is even and a[i] < a[i+1])
    /// 对于condition 2. 同理.
    /// m[i+1]  = 1 if (i is even and a[i] <= a[i+1]) or (i is odd and a[i] >= a[i+1])
    ///         = m[i+1] if (i is even and a[i] > a[i+1]) or (i is odd and a[i] < a[i+1])
    /// 但是两种情况不能混用连贯逻辑，所以可以用一个二元tuple分别记录.
    pub fn max_turbulence_size(a: Vec<i32>) -> i32 {
        let mut m = Vec::new();
        let mut ret = 1;
        // ().0 = condition 1, ().1 = condition 2.
        m.resize(a.len(), (0, 0));
        m[0].0 = 1;
        m[0].1 = 1;
        for index in 1..a.len() {
            let k = index - 1;
            if k % 2 == 1 {
                if a[k] == a[k+1] {
                    m[k+1] = (1, 1);
                } else if a[k] > a[k+1] {
                    m[k+1] = (m[k].0 +1, 1);
                } else {
                    m[k+1] = (1, m[k].1 + 1);
                }
            } else {
                if a[k] == a[k+1] {
                    m[k+1] = (1, 1);
                } else if a[k] > a[k+1] {
                    m[k+1] = (1, m[k].1 + 1);
                } else {
                    m[k+1] = (m[k].0 + 1, 1);
                }
            }
            ret = ret.max(m[k+1].0).max(m[k+1].1);
        }
        ret
    }
}



#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    pub fn longest_turbulent_array_test() {
        assert_eq!(Solution::max_turbulence_size(vec![9,4,2,10,7,8,8,1,9]), 5);
        assert_eq!(Solution::max_turbulence_size(vec![4,8,12,16]), 2);
        assert_eq!(Solution::max_turbulence_size(vec![100]), 1);
        assert_eq!(Solution::max_turbulence_size(vec![9,4,2,10,7,8,8,1,9,4,3,6,7,3,6,3,6,7,3,2,5,7,2,5,3,6,7,2,3,5,7,3,2,5,6,3,5,3,5]), 6);
    }
}



