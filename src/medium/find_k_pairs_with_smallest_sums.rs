use crate::Solution;

use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
struct N1N2Item {
    n1v: i32,
    n2v: i32,
    n2_index: usize,
    sum: i32,
}

impl Ord for N1N2Item {
    fn cmp(&self, other: &Self) -> Ordering {
        other.sum.cmp(&self.sum)
        // self.sum.cmp(&other.sum)
    }
}

impl PartialOrd for N1N2Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.sum.partial_cmp(&self.sum)
        // self.sum.cmp(&other.sum)
    }
}

impl N1N2Item {
    fn new(v1: i32, v2: i32, v2_index: usize) -> Self {
        N1N2Item {
            n1v: v1,
            n2v: v2,
            n2_index: v2_index,
            sum: v1 + v2,
        }
    }
}


impl Solution {
    /// https://leetcode.com/problems/find-k-pairs-with-smallest-sums/
    /// nums1, nums2升序, 不一定长度相等. 找到k个(n1, n2), 使得和最小. 里面数字可以复用.
    /// 这里注意到, 对于一个nums1[i], 它加入结果集合后(假设为nums2[j]), 可以开启的候选集合为 nums1[i] + nums2[j+1]
    /// 其他的没有直接关系.
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        use std::collections::binary_heap::BinaryHeap;
        if nums1.is_empty() || nums2.is_empty() {
            return Vec::new();
        }
        // 下一次要查找的nums范围.
        let mut ret = Vec::new();
        let mut heap = BinaryHeap::new();
        heap.reserve(k as usize);
        for i in 0..nums1.len().min(k as usize) {
            heap.push(N1N2Item::new(nums1[i], nums2[0], 0));
        }

        for _ in 0..k {
            if heap.is_empty() {
                break;
            }
            let item = heap.pop().unwrap();
            ret.push(vec![item.n1v, item.n2v]);
            if item.n2_index + 1 < nums2.len() {
                heap.push(N1N2Item::new(item.n1v, nums2[item.n2_index+1], item.n2_index+1));
            }
        }
        return ret;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    pub fn find_k_pairs_with_smallest_sums_test() {
        assert_eq!(Solution::k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3),
                   vec![
                       vec![1, 2],
                       vec![1, 4],
                       vec![1, 6]
                   ]);
        assert_eq!(Solution::k_smallest_pairs(vec![1, 1, 2], vec![1, 2, 3], 2),
                   vec![
                       vec![1, 1],
                       vec![1, 1],
                   ]);
        assert_eq!(Solution::k_smallest_pairs(vec![1, 2], vec![3], 3),
                   vec![
                       vec![1, 3],
                       vec![2, 3],
                   ]);
        assert_eq!(Solution::k_smallest_pairs(vec![1, 1, 2], vec![1, 2, 3], 10),
                   vec![
                       vec![1, 1],
                       vec![1, 1],
                       vec![2, 1],
                       vec![1, 2],
                       vec![1, 2],
                       vec![2, 2],
                       vec![1, 3],
                       vec![1, 3],
                       vec![2, 3],
                   ]);
    }
}
