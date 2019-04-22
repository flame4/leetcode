use crate::Solution;
use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
struct Counter {
    num: i32,
    frequency: i32,
}

impl PartialOrd for Counter {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.frequency.partial_cmp(&other.frequency)
    }
}

impl Ord for Counter {
    fn cmp(&self, other: &Self) -> Ordering {
        self.frequency.cmp(&other.frequency)
    }
}

impl Counter {
    pub fn new(num: i32, frequency: i32) -> Counter {
        Counter { num, frequency }
    }
}

impl Solution {
    /// https://leetcode.com/problems/top-k-frequent-elements/
    /// 第一步, 构建map, k=num, value = frequency
    /// 第二步, 用最大化堆构建数据.
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::HashMap;
        use std::collections::binary_heap::BinaryHeap;
        let mut ret = Vec::new();
        let mut map = HashMap::new();
        for i in nums {
            if let Some(cnt) = map.get_mut(&i) {
                *cnt += 1;
            } else {
                map.insert(i, 1);
            }
        }

        let mut heap = BinaryHeap::new();
        heap.reserve(k as usize);
        for (num, frequency) in map {
            heap.push(Counter::new(num, frequency));
        }

        for _ in 0..k {
            ret.push(heap.pop().unwrap().num)
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    pub fn top_k_frequent_test() {
        assert_eq!(Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
        assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
    }
}

