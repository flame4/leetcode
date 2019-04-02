use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/decode-ways/
    /// 典型的动态规划例子. 因为某个数字的生效要么它自己生效, 要么和它的前一个数字生效.
    /// 设 m[i]为前i个字符可以构成的组合数.
    /// 则 m[i+1] 由两部分构成. 1. i+1自己 + m[i] (当i+1有效.)
    /// 2. 如果 s[i,i+1]合法, 则 再加上 m[i-1]的所有可能性.
    /// 要注意到一种case, 例如 0000001, 那其实是没有合法表示的.
    /// 在这个动态规划例子下.
    pub fn num_decodings(s: String) -> i32 {
        if s.is_empty() { return 0; }
        let mut m = Vec::new();
        m.resize(s.len() + 1, 0);
        let s_bytes = s.as_bytes();
        if s_bytes[0] == '0' as u8 {
            return 0;
        } else {
            m[1] = 1;
        }
        for i in 2..=s_bytes.len() {
            // early stop
            if m[i - 1] == 0 && m[i - 2] == 0 {
                return 0;
            }

            if s_bytes[i - 1] != ('0' as u8) {
                // case 1成立.
                m[i] += m[i - 1];
            }
            if Solution::check_valid_2char([s_bytes[i - 2], s_bytes[i - 1]]) {
                // case 2 成立.
                // TODO 怎么写可以少一次copy.
                if i - 2 == 0 {
                    m[i] += 1;
                } else {
                    m[i] += m[i - 2];
                }
            }
        }
        m[s.len()]
    }

    fn check_valid_2char(s: [u8; 2]) -> bool {
        if s[0] == ('1' as u8) {
            return true;
        } else if s[0] == ('2' as u8) {
            return s[1] <= ('6' as u8);
        } else {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    pub fn decode_ways_test() {
        assert_eq!(Solution::num_decodings("12".to_string()), 2);
        assert_eq!(Solution::num_decodings("226".to_string()), 3);
        assert_eq!(Solution::num_decodings("206".to_string()), 1);
        assert_eq!(Solution::num_decodings("2063189472389572893".to_string()), 4);
        assert_eq!(Solution::num_decodings("00000001".to_string()), 0);
        assert_eq!(Solution::num_decodings("100000".to_string()), 0);
        assert_eq!(Solution::num_decodings("".to_string()), 0);
        assert_eq!(Solution::num_decodings("1".to_string()), 1);
        assert_eq!(Solution::num_decodings("0".to_string()), 0);
        assert_eq!(Solution::num_decodings("00".to_string()), 0);
        assert_eq!(Solution::num_decodings("000".to_string()), 0);
        assert_eq!(Solution::num_decodings("0000".to_string()), 0);
    }
}

