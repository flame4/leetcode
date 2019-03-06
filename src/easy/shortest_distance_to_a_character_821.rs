use crate::Solution;

impl Solution {
    // 正着迭代一次, 如果一个地方的值错了, 只能是它后面的第一个字母导致的
    // 所以再反着来一次, 取最小就行了.
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut ret: Vec<i32> = Vec::new();
        ret.resize(s.len(), std::i32::MAX - 1);
        let mut min_distince = std::i32::MAX - 1;
        for index in 0..s.len() {
            if s.as_bytes()[index] as char == c {
                min_distince = 0;
            } else {
                min_distince = (min_distince+1).min(std::i32::MAX - 1);
            }
            ret[index] = min_distince.min(ret[index]);
        }
        let mut min_distince = std::i32::MAX - 1;
        // 注意怎么写反向迭代.
        for index in (0..s.len()).rev() {
            if s.as_bytes()[index] as char == c {
                min_distince = 0;
            } else {
                min_distince = (min_distince+1).min(std::i32::MAX - 1);
            }
            ret[index] = min_distince.min(ret[index]);
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    pub fn shortest_to_char_test() {
        assert_eq!(Solution::shortest_to_char("loveleetcode".to_string(), 'e'), vec![3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0]);
    }
}