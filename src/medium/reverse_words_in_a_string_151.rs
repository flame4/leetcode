use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/reverse-words-in-a-string/
    /// 先翻转整体，再切分对每个单词翻转即可.
    pub fn reverse_words(s: String) -> String {
        if s.trim().is_empty() {return "".to_string();}
        let mut s = {s};
        let mut ret = String::new();
        unsafe {
            s.as_mut_vec().reverse();
        }
        for word in s.trim().split_whitespace() {
            let mut word = word.to_string();
            unsafe { word.as_mut_vec().reverse(); }
            ret += &word;
            ret += " ";
        }
        ret.remove(ret.len()-1);
        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    pub fn reverse_words_test() {
        assert_eq!("blue is sky the", Solution::reverse_words("the sky is blue".to_string()));
        assert_eq!("world! hello", Solution::reverse_words("  hello world!  ".to_string()));
        assert_eq!("example good a", Solution::reverse_words("a good   example".to_string()));
    }
}
