use crate::Solution;

impl Solution {
    pub fn is_valid_parentheses(s: String) -> bool {
        let mut ret = true;
        let mut stack = Vec::new();
        for c in s.chars() {
            if Solution::judge_open(c) {
                stack.push(c)
            } else {
                let tmp = stack.pop().unwrap_or('#');
                if !Solution::judge_match(tmp, c) {
                    return false;
                }
            }
        }
        stack.is_empty()
    }

    fn judge_open(c: char) -> bool {
        c == '(' || c == '{' || c == '['
    }

    fn judge_match(c_open: char, c_close: char) -> bool {
        match c_open {
            '(' => c_close == ')',
            '{' => c_close == '}',
            '[' => c_close == ']',
            _ => false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    pub fn is_valid_parentheses_test() {
        assert_eq!(Solution::is_valid_parentheses("()".to_string()), true);
        assert_eq!(Solution::is_valid_parentheses("(){}[]".to_string()), true);
        assert_eq!(Solution::is_valid_parentheses("(]".to_string()), false);
        assert_eq!(Solution::is_valid_parentheses("([)]".to_string()), false);
        assert_eq!(Solution::is_valid_parentheses("{[()]}".to_string()), true);
    }
}
