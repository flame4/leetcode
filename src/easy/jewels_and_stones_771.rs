use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
        let mut num = 0;
        let bytes = j.into_bytes();
        let mut jewels_set = HashSet::new();

        // TODO 运用迭代器来优化写法
        for i in 0..bytes.len() {
            jewels_set.insert(bytes[i]);
        }

        let bytes = s.into_bytes();
        for i in 0..bytes.len() {
            if jewels_set.contains(&bytes[i]) {
                num += 1
            }
        }
        num
    }
}
