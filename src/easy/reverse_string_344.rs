use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/reverse-string/
    /// 翻转字符串. 头尾调换即可.
    pub fn reverse_string(s: &mut Vec<char>) {
        if s.is_empty() { return; }
        let mut start = 0;
        let mut end = s.len() - 1;
        while start < end {
            let tmp = s[start];
            s[start] = s[end];
            s[end] = tmp;
            start += 1;
            end -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn reverse_string_test() {
        let mut raw = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&mut raw);
        assert_eq!(raw, vec!['o', 'l', 'l', 'e', 'h']);

        raw = vec![];
        Solution::reverse_string(&mut raw);
        assert_eq!(raw, vec![]);
    }
}
