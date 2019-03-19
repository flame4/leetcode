use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/non-decreasing-array/
    /// 只改变最多一个元素，令数组变为非降低数组。
    /// 思路1：贪心判断. 以两个元素为窗口, 递归的判断这个数组是不是符合规范.
    /// 在num[i] > num[i+1]的情况下, 如果能减少num[i]，优先这么做，因为增大num[i+1]可能使得num[i+1] > num[i+2]
    /// 但是num[i]最少只能缩小到num[i-1], 如果不满足条件， 我们只能令num[i+1] = num[i], 然后迭代的进入下一次判断.
    /// 记录必须要操作的次数，直到其超过1.
    /// 但是这个思路困难的一点是,一旦num[i] < num[i+1], 不进入下一伦判断, 我们不知道是不是该缩小num[i+1]. (没写出来)
    /// 思路2：
    /// 从思路1的思考过程中, 我们可以看到, 因为限定了一次操作, 我们不得不一次看3个值来决定所有的情况, 类似的，如果限定了两次操作，必须要看4个值.
    /// 用动态规划的思路, 描述问题如下.
    /// a[i] = x 必须要经过几次操作才能满足要求.
    /// a[i+1] =    | if num[i] <= num[i+1] => a[i+1] = (true, x)
    ///             | if num[i] > num[i+1] | if num[i-1] < num[i+1] => a[i+1] = (true, x+1) and num[i] = num[i-1]
    ///             |                      | else  a[i+1] = (true, x+1) and a[i+1] = a[i]
    /// 这个思路在rust内，需要遍历num的时候再修改num的值，既需要获取可用引用，又需要不可用引用，编译不通过，不是个安全的好方案。
    /// 思路3：直接思考这个数组要满足这个条件，需要满足什么本质条件.
    /// 本质条件为，不能存在两组num[i] > num[i+1], 否则必须要调整至少两次.
    /// 那么我们遍历数组, 只要发现两组，就失败. 否则，就判断那唯一的一组是不是可以通过修改来满足条件，而这只需要看其周围的几个数字就行了.
    ///
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let nums = { nums };
        if nums.len() < 2 { return true; }
        let mut illegal_index = None;
        for (index, _) in nums.iter().enumerate() {
            if index == 0 { continue; }
            if nums[index] < nums[index - 1] {
                if illegal_index.is_some() {
                    return false;
                } else {
                    illegal_index = Some(index)
                }
            }
        }
        (illegal_index.is_none())
            || (illegal_index.unwrap() == 1)
            || (illegal_index.unwrap() == nums.len() - 1)
            || (nums[illegal_index.unwrap() - 2] < nums[illegal_index.unwrap()])
            || (nums[illegal_index.unwrap() - 1] < nums[illegal_index.unwrap() + 1])
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    pub fn check_possibility_test() {
        assert_eq!(Solution::check_possibility(vec![4, 2, 3]), true);
        assert_eq!(Solution::check_possibility(vec![4, 2, 1]), false);
        assert_eq!(Solution::check_possibility(vec![3, 4, 2, 3]), false);
        assert_eq!(Solution::check_possibility(vec![2, 3, 3, 2, 4]), true);
    }
}