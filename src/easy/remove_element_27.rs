use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/remove-element/
    /// 去除所有给定的元素, 并且返回后来的长度. O(1)空间.
    /// 不断把重复元素往后放, 最后截断一次就行了.
    /// 从后往前遍历容易想的清楚.
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.is_empty() { return 0;}
        let mut valid = nums.len() - 1;
        let mut poineer = nums.len() - 1;
        while poineer >= 0 {
            if nums[poineer] == val {
                while valid > poineer && nums[valid] == val {
                    valid -= 1;
                }
                nums.swap(valid, poineer);
            }
            if poineer == 0 { break; }
            poineer -= 1;
        }
        if nums[valid] == val {
            nums.truncate(valid);
        } else {
            nums.truncate(valid + 1);
        }
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    pub fn max_area_test() {
        let mut a = vec![3, 2, 2, 3];
        assert_eq!(Solution::remove_element(&mut a, 3), 2);
        let mut a = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(Solution::remove_element(&mut a, 2), 5);
        let mut a = vec![2];
        assert_eq!(Solution::remove_element(&mut a, 3), 1);
    }
}
