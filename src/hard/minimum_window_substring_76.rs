use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/minimum-window-substring/
    /// 给定S和T，在O(n)时间找出一个最小窗口, S包含T的所有字母.
    /// 我们先简化问题假定S和T内没有重复字母, 那么问题非常好解决, 维护一个从左到右的可变长窗口,
    /// 用一个32bit的位数来记录是否出现过, 然后每次扩容右边窗口直到满足S含有T, 再收缩左边直到满足的最小边界.
    /// 然后继续扩右边缩左边找到下一个窗口, 不断比较最小窗口, 这个是O(n)的, 只要保证每次的比较都是o(1)的.
    /// 那么麻烦的是, 如果S和T内有重复数据, 用一个整数表达不出来了. 那怎么办呢, 换用其他的数据结构保证o(1)即可.
    /// 可以想到的, 只有Map啦.
    pub fn min_window(s: String, t: String) -> String {
        use std::collections::HashMap;
        // v.0代表实际上t内有多少这个字母, v.1表示在当前窗口下已经有多少个.
        let mut t_map: HashMap<u8, [i32; 2]> = HashMap::new();
        // 记录t内有多少不同的字母.
        let mut t_chars = 0;
        for c in t.as_bytes() {
            if let Some(v) = t_map.get_mut(c) {
                v[0] += 1;
            } else {
                t_map.insert(*c, [1, 0]);
                t_chars += 1;
            }
        }
        let s_bytes = s.as_bytes();
        let mut left = 0;
        let mut right = 0;
        // 记录多少个字母已经满足在T内, 当到达t_chars时候, 证明区间满足.
        let mut meet_chars = 0;
        let mut ret = "".as_bytes();
        // 区间选择为 [left, right]
        // 进入的当前区间表示没有处理过.
        while right < s_bytes.len() {
            if let Some(v) = t_map.get_mut(&s_bytes[right]) {
                v[1] += 1;
                if v[1] == v[0] {
                    meet_chars += 1;
                }

                if meet_chars == t_chars {
                    // 全集合已经match, 开始处理左边半区的收缩.
                    while meet_chars == t_chars {
                        if let Some(u) = t_map.get_mut(&s_bytes[left]) {
                            u[1] -= 1;
                            if u[1] < u[0] {
                                meet_chars -= 1;
                            }
                        }
                        left += 1;
                    }
                    // 收缩完毕, 比较.
                    if ret.len() == 0 || right - left + 2 < ret.len() {
                        ret = s.as_bytes()[(left - 1)..=right].as_ref();
                    }
                }
            }
            right += 1
        }

        unsafe { String::from_utf8_unchecked(ret.to_owned()) }
    }
}



#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn min_window_test() {
        assert_eq!(Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()), "BANC".to_string());
        assert_eq!(Solution::min_window("ABAACBAB".to_string(), "ABC".to_string()), "ACB".to_string());
        assert_eq!(Solution::min_window("A".to_string(), "B".to_string()), "".to_string());
        assert_eq!(Solution::min_window("A".to_string(), "A".to_string()), "A".to_string());
        assert_eq!(Solution::min_window("AA".to_string(), "AAAA".to_string()), "".to_string());
    }
}
