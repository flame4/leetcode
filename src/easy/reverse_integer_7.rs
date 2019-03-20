use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/reverse-integer
    /// 最麻烦的是考虑多次溢出的情况
    pub fn reverse_integer(x: i32) -> i32 {
        // abs(x) not work.
        if x == -2147483648 { return 0; }
        let mut x = x;
        let sign;
        if x < 0 {
            sign = -1;
            x = -x;
        } else {
            sign = 1;
        }
        let mut ret = 0;
        while x != 0 {
            if (ret > 2147483647/10) || ((ret == 2147483647) && (x%10>7)) {
                return 0;
            }
            ret = ret * 10 + x % 10;
            x /= 10;
        }
        ret * sign
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    pub fn reverse_integer_test() {
        assert_eq!(Solution::reverse_integer(123), 321);
        assert_eq!(Solution::reverse_integer(-123), -321);
        assert_eq!(Solution::reverse_integer(0), 0);
        assert_eq!(Solution::reverse_integer(std::i32::MAX), 0);
        assert_eq!(Solution::reverse_integer(std::i32::MIN), 0);
    }
}
