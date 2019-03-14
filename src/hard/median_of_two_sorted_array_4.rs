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
    #[allow(unused_assignments)]
    fn find_median_sorted_noempty_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // 注释别名为m
        let nums1_size = nums1.len();
        // 注释别名为n
        let nums2_size = nums2.len();
        // 在i内寻找的区间, index start.
        let mut nums1_start = 0;
        // 在i内寻找的区间, index end.
        let mut nums1_end = nums1.len() - 1;
        // i 对应的index
        let mut i = 0;
        let ret: f64;
        // j 对应的index.
        // 当在nums1内的坐标为i的时候, j = (m+n/2)-i-1.
        let mut j = 0; // cursor for nums2
        loop {
            i = (nums1_start + nums1_end) / 2;
            j = (nums1_size + nums2_size) / 2 - i - 1;

            if nums1_start > nums1_end {
                // 意味着nums1内不存在中间, 要么太大要么太小.
                break;
            }

            if nums1[i] <= nums2[j + 1] && nums2[j] <= nums1[i + 1] {
                break;
            } else if nums1[i] > nums2[j + 1] {
                // 在nums1内, 这个数字太大了. 所以减少i.
                nums1_end = i - 1;
            } else {
                nums1_start = i + 1;
            }
        }
        if (nums1_size + nums2_size) % 2 == 0 {
            // 偶数应该取两者中间和.
            if nums1_end < 0 {
                ret = nums2[(nums2_size + nums1_size) / 2 - 1] as f64;
            } else if nums1_start > nums1_size {
                ret = nums2[(nums2_size - nums1_size) / 2 - 1] as f64;
            } else {
                ret = (nums1[i] as f64 + nums2[j] as f64) / 2f64;
            }
        } else {
            // 奇书应该取j.
            if nums1_end < 0 {
                ret = nums2[(nums2_size + nums1_size) / 2 - 1] as f64;
            } else if nums1_start > nums1_size {
                ret = nums2[(nums2_size - nums1_size) / 2 - 1] as f64;
            } else {
                ret = (nums1[i] as f64 + nums2[j] as f64) / 2f64;
            }
        }
        ret
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

