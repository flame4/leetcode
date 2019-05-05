use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/product-of-array-except-self/
    /// 给定一个数组，返回一个数组，output[i]等于所有的数字积, 除了nums[i].
    /// 简单的方法很简单, 看如何高效处理这个问题.
    /// 首先, 必须是线性时间和o(1)空间, 其次, 不应该使用除法, 因为除法比乘法低效很多.
    /// 我们只需要思考我们可以干什么就可以了.
    /// 对于我们先看num[0], 我们看到nums[0]的时候, 给Output[0]只能贡献一个1的有效值。
    /// num[1], 对于output[1]的有效值是一个nums[0]
    /// 所以以此类推, 我们第一遍遍历可以得到下列的output列表.
    /// output = [1, nums[0], nums[0] * nums[1], ... , nums[0] * nums[1] * ... * nums[n-2]]
    /// nums = [nums[0], nums[1], ..., nums[n-1]]
    ///
    /// 发现了一些很奇妙的东西，这个过程我们只需要再倒着做一遍，就可以得到最后的结果了.
    /// 但是我们的空间会超限，没关系，复用nums！
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = {nums};
        let mut output = Vec::new();
        output.resize(nums.len(), 1);
        for index in 1..nums.len() {
            output[index] = output[index - 1] * nums[index - 1];
        }
        for index in (0..(nums.len()-1)).rev() {
            output[index] = output[index] * nums[index + 1];
            nums[index] *= nums[index + 1];
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn product_except_self() {
        assert_eq!(Solution::product_except_self(vec![1,2,3,4]), vec![24, 12, 8, 6]);
        assert_eq!(Solution::product_except_self(vec![3,6,7,6,4,6,6,8,5]),
                   vec![1451520,725760,622080,725760,1088640,725760,725760,544320,870912]);
        assert_eq!(Solution::product_except_self(vec![3]), vec![1]);
    }
}
