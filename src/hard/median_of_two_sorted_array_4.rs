use crate::Solution;


impl Solution {
    /// https://leetcode.com/problems/median-of-two-sorted-arrays/
    /// 在O(log(m+n))内完成中位数查询. nums1, nums2 不会同时为空.
    /// 思路1:
    /// 问题的核心难点在于在问题的限定时间内, 首先我们不能遍历任何数组, 那么只能进行二分查找.
    /// 问题的关键在于, 因为median限定了数量, 所以取定一个位置, 另一个就也定下了.
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        match (nums1.len(), nums2.len()) {
            (0, 0) => return 0f64,
            (0, _) => {
                if nums2.len() % 2 == 0 {
                    return nums2[nums2.len() / 2] as f64 / 2f64 + nums2[nums2.len() / 2 - 1] as f64 / 2f64;
                } else {
                    return nums2[nums2.len() / 2] as f64;
                }
            }
            (_, 0) => {
                if nums1.len() % 2 == 0 {
                    return nums1[nums1.len() / 2] as f64 / 2f64 + nums1[nums1.len() / 2 - 1] as f64 / 2f64;
                } else {
                    return nums1[nums1.len() / 2] as f64;
                }
            }
            _ => {
                if nums1.len() > nums2.len() {
                    Solution::find_median_sorted_noempty_arrays(nums2, nums1)
                } else {
                    Solution::find_median_sorted_noempty_arrays(nums1, nums2)
                }
            }
        }
    }

    /// nums1.len <= nums2.len
    /// 好处是我们确保最后的下界一定在nums2内, 方便我们处理nums1已经遍历完了的边界情况.
    /// 要注意循环的停止条件是找到两个位置i,j. nums1中i前面的都小于等于j. nums2中j前面的都小于等于j.
    /// 这里在算offset的时候会有点糊涂, 减少思维负担的做法是严格定义清楚i,j的含义, 代码中严格卡自己的定义来.
    /// 需要搞清楚的有三件事情:
    /// 1. 当一边选择为i的时候， 另一边的j计算规则
    /// 这里需要定义,如果选择i为区分点, num[i]属于右边的集合. 也就是左边有i个数,右边有m-i个.
    /// 则两个数组的情况下, 左边一共有i+j个数字.则 median的定义为左边最大的小于等于右边最小的.
    /// 借用一个数组的计算中点规则(偶数或者奇数都适用).  i+j = m+n-i-j. j = (m+n)/2 - i;
    /// 2. 当m+n is odd, median其实只有一个数字, 怎么在双数组内表征这种情况? 核心问题是median这个的定义.
    /// 在第一个问题内已经回答, 计算虽然基于偶数情况, 但是因为计算机整数除法的特征, 也可以用作奇数.
    /// 此外, median要求 nums1[i-1] <= nums2[j] and nums2[j-1] <= nums1[i];
    /// 这里也要注意i-1, j-1的合理存在条件.
    /// 3. 处理nums1到了边界的情况.
    /// 一种手法: 直接判断每次取的中点是不是小于end,来判断begin和end是不是一样的.
    #[allow(unused_assignments)]
    fn find_median_sorted_noempty_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // 注释别名为m
        let nums1_size = nums1.len();
        // 注释别名为n
        let nums2_size = nums2.len();
        // 在i内寻找的区间, index start.
        let mut nums1_start = 0;
        // 在i内寻找的区间, index end.
        let mut nums1_end = nums1.len();
        // i 对应的index
        let mut i = 0;
        let max_left: f64;
        let min_right: f64;
        // j 对应的index.
        let mut j = 0; // cursor for nums2
        while nums1_start <= nums1_end {
            i = (nums1_start + nums1_end) / 2;
            j = (nums1_size + nums2_size + 1) / 2 - i;
            //println!("i = {}, j = {}", i, j);

            if i < nums1_end && nums2[j - 1] > nums1[i] {
                // i太小了. 为什么这里只需要判断 i < end?
                // 这个判断真正起作用是在i只有一个的时候.
                nums1_start = i + 1;
            } else if i > nums1_start && nums1[i - 1] > nums2[j] {
                nums1_end = i - 1;
            } else {
                // i 收敛到足够判断.
                if i == 0 {
                    max_left = nums2[j - 1] as f64;
                } else if j == 0 {
                    max_left = nums1[i - 1] as f64;
                } else {
                    max_left = nums2[j - 1].max(nums1[i - 1]) as f64;
                }
                if (nums1_size + nums2_size) % 2 == 1 { return max_left; }

                if i == nums1_size {
                    min_right = nums2[j] as f64;
                } else if j == nums2_size {
                    min_right = nums1[i] as f64;
                } else {
                    min_right = nums2[j].min(nums1[i]) as f64;
                }
                return (min_right + max_left) / 2f64 ;
            }
        }
        unreachable!()
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn find_median_sorted_arrays_test() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3], vec![2]), 2f64);
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3], vec![2, 4]), 2.5f64);
        assert_eq!(Solution::find_median_sorted_arrays(
            vec![1, 3, 4, 5, 7, 8, 9],
            vec![4, 6, 8, 11, 13],
        ), 6.5f64);
    }
}

