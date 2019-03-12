use crate::Solution;

impl Solution {
    // TODO 直接在原始内存上操作.
    pub fn reverse_words_iii(s: String) -> String {
        let mut ret = String::new();
        for i in s.split_whitespace() {
            unsafe {
                let mut ss = i.to_string();
                ss.as_mut_vec().reverse();
                ret.push_str(&ss);
                ret.push(' ');
            }
        }
        if ret.len() > 0 {
            ret.remove(ret.len() - 1);
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    pub fn reverse_words_iii_test() {
        assert_eq!(Solution::reverse_words_iii("Let's take LeetCode contest".to_string()), "s'teL ekat edoCteeL tsetnoc".to_string());
        assert_eq!(Solution::reverse_words_iii("".to_string()), "".to_string())
    }
}