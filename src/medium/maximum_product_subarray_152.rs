use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/maximum-product-subarray/
    /// 最大连续子序列的乘积. 有正数有负数.
    /// 注意要求是至少含有一个元素. 所以假设全是负数, 就应该返回最大的负数.
    /// 这类问题和house robber一样, 递归表达式里面一定有一项自己.
    ///
    /// 这个题用动归即可, 但是要保存最小的元素.  m: Vec<[i32;2]>
    /// m[i]表示以a[i]为结尾的结果, 0代表最大正数, 1代表最小负数.
    /// 要注意, 序列一定是连续的. 所以m[i]的定义一定是以a[i]为结尾的最值, 而不能是到a[i]目前为止的最值.
    /// 否则考虑 1,2,3,-4,5,6,7这个数组. 因为有了-4, a[3].0如果是6, 那么后续迭代结果就变成了1,2,3,5,6,7
    /// 数据的连续性没有在定义里保证.
    ///
    /// if a[i+1] >= 0
    /// m[i+1].0 = max(m[i].0 * a[i+1], a[i+1])
    /// m[i+1].1 = m[i].1 * a[i+1]
    ///
    /// if a[i+1] < 0
    /// m[i+1].0 = m[i].1 * a[i+1]
    /// m[i+1].1 = min(m[i].0 * a[i+1], a[i+1])
    ///
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut m = Vec::new();
        m.resize(nums.len(), (0, 0));
        m[0].0 = 0.max(nums[0]);
        m[0].1 = 0.min(nums[0]);
        let mut ret: i32 = nums[0];


        for index in 1..nums.len() {
            if nums[index] >= 0 {
                m[index].0 = (m[index - 1].0 * nums[index]).max(nums[index]);
                m[index].1 = m[index - 1].1 * nums[index];
            } else {
                m[index].0 = m[index - 1].1 * nums[index];
                m[index].1 = (m[index - 1].0 * nums[index]).min(nums[index]);
            }
            ret = ret.max(m[index].0)
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    pub fn sliding_window_maximum_test() {
        assert_eq!(Solution::max_product(vec![1, 3, -1, -3, 5, 3, 6, 7]), 5670);
        assert_eq!(Solution::max_product(vec![-1]), -1);
        assert_eq!(Solution::max_product(vec![-1, 0]), 0);
        assert_eq!(Solution::max_product(vec![2,3,-2,4]), 6);
        assert_eq!(Solution::max_product(vec![-2, 0, -1]), 0);
    }
}

