use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/regular-expression-matching/
    /// 对这个问题来说, naive的case处理不好做, 因为:
    /// 1. *代表多少个数字, 是个问题.
    /// 2. ".*" 能代表无数任意长度字符
    /// 3. 如果 ".*"后面还跟着其他字符, 则s也必须要满足后面的模式才可以.
    /// 这种后面信息强行依赖前面的, 可以用动态规划来做.
    pub fn is_match(s: String, p: String) -> bool {
        Solution::is_match_in_u8vec(s.as_bytes(), p.as_bytes())
    }


    /// 这里需要注意的是, 如果p里面有一个.*, 那么s[i]的判断就不能仅仅依赖前面来判断是否成立了.
    /// 传统的从前到后的递归方式变得不再可用.
    /// 从题解中可以学到的新技巧是, 这个递归将从后往前做. 因为当遇到一个.*的时候, 我们其实是可以判定后面
    /// 是不是满足要求, 可以先删除掉后面满足要求的部分, 剩下的用 .* 来匹配.
    /// m[i][j] 表示从s[i]向后的字符和从p[j]的字符向后能否组成合适字符串.
    /// 递归表达式如下(注意i,j的含义和其在p数组内的偏移概念是否可以对应起来):
    /// m[i][j] 需要分情况讨论. 首先要明确, 以*开始的字符串那个*匹配为空. 每个m[i][j]需要看p[j+1]是不是star.
    /// 如果不是, 那么只需要简单的匹配规则即可.
    /// m[i][j] = m[i+1][j+1] and (s[i]=p[j] or p[j]='.')
    /// 如果是(利用递归的思路很重要),要么这个*完全用不到, 要么它可以匹配到m[i+1][j].
    /// m[i][j] = m[i][j+2] or ()
    fn is_match_in_u8vec(s: &[u8], p: &[u8]) -> bool {
        let point_u8 = '.' as u8;
        let star_u8 = '*' as u8;
        let mut m = Vec::new();
        let mut tmp = vec![false];
        tmp.resize(p.len() + 1, false);
        for _ in 0..=s.len() { m.push(tmp.clone()) }
        // 显然, 空和空可以互相组成.
        // 但是, 空和p[j-1]是不是可以组成, 是有待考量的, 所以, i的遍历需要从s.len()开始, 而不是s.len()-1
        m[s.len()][p.len()] = true;
        for i in (0..=s.len()).rev() {
            for j in (0..p.len()).rev() {
                let first_match = i < s.len() && (s[i] == p[j] || p[j] == point_u8);
                if j + 1 < p.len() && p[j + 1] == star_u8 {
                    m[i][j] = m[i][j+2] || (first_match && m[i+1][j])
                } else {
                    m[i][j] = first_match && m[i+1][j+1]
                }
            }
        }
        m[0][0]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    pub fn is_match_test() {
        assert_eq!(Solution::is_match("aaaaa".to_string(), ".*.*".to_string()), true);
        assert_eq!(Solution::is_match("aaaaa".to_string(), "a.*".to_string()), true);
        assert_eq!(Solution::is_match("aaaaa".to_string(), ".*b".to_string()), false);
        assert_eq!(Solution::is_match("".to_string(), "".to_string()), true);
        assert_eq!(Solution::is_match("".to_string(), ".*".to_string()), true);
        assert_eq!(Solution::is_match("".to_string(), "*".to_string()), false);
        assert_eq!(Solution::is_match("".to_string(), "*".to_string()), false);
        assert_eq!(Solution::is_match("aab".to_string(), "c*a*b".to_string()), true);
        assert_eq!(Solution::is_match("mississippi".to_string(), "mis*is*p*.".to_string()), false);
    }
}
