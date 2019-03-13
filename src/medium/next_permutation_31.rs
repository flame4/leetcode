use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/next-permutation/
    /// 考虑, 什么情况下是一个序列回归到1,2,3,4这样的递增序列的时候?
    /// 是一个数组处于完全的递减序列排序的时候.
    /// 那么对于最右边的处于递减子序列的情况, 下一个一定是这个序列回归到正常的顺序.
    /// 然后这个序列前面那个数字换后面序列第一个比它大的, 如果没有, 则说明这个数组完全倒序.
    /// 下个序列应该是把他们正过来即可.
    /// 要注意的是,不保证内部没有相同的数字,
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if nums.is_empty() { return; }
        let mut last_permutation_start_pos = 0;
        for i in 0..nums.len() - 1 {
            if nums[i] < nums[i+1] {
                last_permutation_start_pos = i+1;
            }
        }
        if last_permutation_start_pos == 0 {
            nums.reverse();
            return;
        } else {
            nums[last_permutation_start_pos..].reverse();
        }
        // TODO 这里也可以优化, 在第一次查找的时候就可以记录下来.
        for i in last_permutation_start_pos..nums.len() {
            if nums[i] > nums[last_permutation_start_pos-1] {
                nums.swap(i, last_permutation_start_pos-1);
                break;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn next_permutation_test() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        Solution::next_permutation(&mut v);
        assert_eq!(vec![1,2,3,4,6,5], v.as_mut());
        Solution::next_permutation(&mut v);
        assert_eq!(vec![1,2,3,5,4,6], v.as_mut());
        Solution::next_permutation(&mut v);
        assert_eq!(vec![1,2,3,5,6,4], v.as_mut());
        Solution::next_permutation(&mut v);
        assert_eq!(vec![1,2,3,6,4,5], v.as_mut());
        Solution::next_permutation(&mut v);
        assert_eq!(vec![1,2,3,6,5,4], v.as_mut());
        Solution::next_permutation(&mut v);
        assert_eq!(vec![1,2,4,3,5,6], v.as_mut());

        let mut v = vec![];
        Solution::next_permutation(&mut v);
        let a : Vec<i32> = vec![];
        assert_eq!(a, v.as_mut());

        let mut v = vec![1];
        Solution::next_permutation(&mut v);
        assert_eq!(vec![1], v.as_mut());
    }
}

