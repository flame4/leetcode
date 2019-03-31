use crate::Solution;
use std::fmt::Debug;

impl Solution {
    pub fn quicksort<T: Ord + Debug>(nums: &mut Vec<T>) {
        if nums.is_empty() { return; }
        Solution::quicksort_internal(nums, 0, nums.len() - 1);
    }

    fn quicksort_internal<T: Ord + Debug>(nums: &mut Vec<T>, start: usize, end: usize) {
        if start >= end {
            return;
        }
        let index = Solution::quicksort_q(nums, start, end);
        if index > 1 {
            Solution::quicksort_internal(nums, start, index - 1);
        }
        Solution::quicksort_internal(nums, index + 1, end);
    }

    fn quicksort_q<T: Ord + Debug>(nums: &mut Vec<T>, start: usize, end: usize) -> usize {
        let mut i = start;
        let mut j = end;
        while i < j {
            while i < j && *(nums.get(i).unwrap()) <= *(nums.get(j).unwrap()) {
                j -= 1;
            }
            nums.swap(i, j);
            while i < j && *(nums.get(i).unwrap()) <= *(nums.get(j).unwrap()) {
                i += 1;
            }
            nums.swap(i, j);
        }
        i.min(j)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn quicksort_test() {
        let mut v = vec![4, 2, 0, 3, 2, 5];
        Solution::quicksort(&mut v);
        assert_eq!(v, vec![0, 2, 2, 3, 4, 5]);

        let mut v: Vec<i32> = vec![];
        Solution::quicksort(&mut v);
        assert_eq!(v, vec![]);

        let mut v = vec![3, 1, 2, 1, 2, 1, 3];
        Solution::quicksort(&mut v);
        assert_eq!(v, vec![1, 1, 1, 2, 2, 3, 3]);

        let mut v = vec![4, 2, 0, 3, 2, 5];
        Solution::quicksort(&mut v);
        assert_eq!(v, vec![0, 2, 2, 3, 4, 5]);

        let mut v = vec![0, 0, 2, 2];
        Solution::quicksort(&mut v);
        assert_eq!(v, vec![0, 0, 2, 2]);

        let mut v = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        Solution::quicksort(&mut v);
        assert_eq!(v, vec![0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 3]);

        let mut v = vec![87, 3, 4, 34, 6, 3, 3, 6, 54];
        Solution::quicksort(&mut v);
        assert_eq!(v, vec![3, 3, 3, 4, 6, 6, 34, 54, 87]);
    }
}

