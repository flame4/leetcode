use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
        let mut num = 0;
        let bytes = j.into_bytes();
        let mut jewels_set = HashSet::new();

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

    // 网上看到的两行rust
    pub fn num_jewels_in_stones_2line(j: String, s:String) -> i32 {
        // TODO 为什么一定要显式声明?
        let jewels: HashSet<char> = j.chars().collect();
        s.chars().filter(|ch| jewels.contains(ch)).count() as i32
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_jewels_in_stones_1() {
        let jewels = String::from("aA");
        let stones = String::from("aAAAbbbb");
        assert_eq!(4, Solution::num_jewels_in_stones(jewels, stones));

        let jewels = String::from("z");
        let stones = String::from("ZZ");
        assert_eq!(0, Solution::num_jewels_in_stones(jewels, stones));

        let jewels = String::from("");
        let stones = String::from("");
        assert_eq!(0, Solution::num_jewels_in_stones(jewels, stones));
    }
}
