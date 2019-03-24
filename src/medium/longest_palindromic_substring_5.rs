use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/longest-palindromic-substring/
    /// 最...子...系列问题. 典型的动态规划解决思路.
    /// 这个问题假设定义s[i]是以i为结尾的最长回文子串, s[i+1]不容易直接从s[i]推导出来, 因为s[i+1]
    /// 也有可能和前面一部分构成最大子串的前面部分. 考虑极端情况,
    /// 1. "abbbb"
    /// 2. "bababd"
    /// 3. "abcbabc"
    /// 会发现每个字母在找前面关于自己出现的最大子串的时候，都可能会其前面的任意点字母构成.
    /// 动态规划的解法是二维的. 定义 m(i,j)是回文子串.
    /// m(i, i) = true, m(i, i+1) = (s[i] == s[i+1])
    /// m(i-1, j+1) = m(i, j) and s[i-1] == s[j+1]
    /// 要注意对于这种从中间往两边扩展的数组, 怎么写循环.本质上是要改变循环的行为，
    /// 普通的循环是两个变量同方向循环，这个要两个变量逆方向循环，才能做到不断往外扩大。
    /// 48ms, faster than 30.4%
    pub fn longest_palindrome(s: String) -> String {
        if s.is_empty() { return "".to_string(); }
        let mut m = Vec::new();
        for _ in 0..s.len() {
            let mut v = Vec::new();
            v.resize(s.len(), false);
            m.push(v);
        }
        let s_byte = s.as_bytes();
        let mut max = 1;
        let mut max_start = 0;
        let mut max_end = 0;

        for i in 0..s.len() {
            m[i][i] = true;
            if i == s.len() - 1 { break; }
            if s_byte[i] == s_byte[i + 1] {
                m[i][i + 1] = true;
                max = 2;
                max_start = i;
                max_end = i + 1;
            }
        }

        // dp
        if s.len() > 2 {
            for i in (0..(s.len() - 2)).rev() {
                for j in (i + 2)..s.len() {
                    m[i][j] = m[i + 1][j - 1] && s_byte[i] == s_byte[j];
                    if j - i + 1 > max && m[i][j] {
                        max = j - i + 1;
                        max_start = i;
                        max_end = j;
                    }
                }
            }
        }
        String::from_utf8(s_byte[max_start..=max_end].to_owned()).unwrap_or("".to_string())
    }

    /// 单数回文和双数回文的判定不同是处理起来比较麻烦的.
    /// 一个巧妙的方法是, 在每一个间隔处插入一个"#", 会让所有的情况都变成单数回文.
    /// 这样可以在O(n^2)的时间内, 从任意点向两端扩容出去直到找到最长的回文子串.
    /// 这个处理被叫做Manacher(马拉车算法的预处理).
    /// 20ms, faster than 32.22%.
    pub fn longest_palindrome_2(s: String) -> String {
        let s = Solution::pre_deal_palindrome(s);
        let mut max_pos = 0;
        let mut max_radius = 0;
        for i in 0..s.len() {
            let mut radius = 0;
            while (i - radius) >= 1 && (i + radius + 1) < s.len() {
                if s.as_bytes()[i - radius - 1] == s.as_bytes()[i + radius + 1] {
                    radius += 1;
                } else {
                    break;
                }
            }
            if radius > max_radius {
                max_pos = i;
                max_radius = radius;
            }
        }
        String::from_utf8(s.as_bytes()[(max_pos - max_radius)..=(max_pos + max_radius)]
            .to_vec()).unwrap().replace("#", "")
    }


    /// 马拉车算法.
    /// http://www.cnblogs.com/grandyang/p/4475985.html
    /// 定义m[i]为以i为中心的字符串的半径. 只有自己的半径为0.
    /// 算法从左向右扩展, 有两个变量记录当前访问到的能延伸到最右端的(没说是当前最长的)的index和能访问到的最右边位置.
    /// 分别为 most_right_center, most_right_index
    /// m[i] = mri > i ? min(m[2*mrc - i], mri-i) : 1;
    ///               j=i的对称   mrc             i          mri
    ///     ↓         ↓            ↓              ↓           ↓
    /// ---------------------------------------------------------------
    /// 假设mrc已知. i只有两种情况, mri > i,  mri <= i.
    /// mri > i的情况下, j是i的对称. 假设i + m[j] < mri, 那么由mrc的对称性可知道, m[i] = m[j].
    /// 否则m[i]到mri的位置是不用比的, 只需要从mri往后比.
    /// 因为mri只被扩展一次, 所以是o(n)的算法复杂度.
    /// mri == i 的情况, 没有其他可利用的信息, 只能从i往后拓展.
    /// mri > i的情况, 只能是mri = i - 1. 因为如果mri = i - 2, 那么mri最小也应该是i-1.
    /// 0ms, faster than 100%.
    pub fn longest_palindrome_3(s: String) -> String {
        let s = Solution::pre_deal_palindrome(s);
        let mut m = Vec::new();
        m.resize(s.len(), 0);
        // 当前已经控制到的右边界.
        let mut most_right_index = 0;
        let mut most_right_center = 0;
        for i in 0..s.len() {
            // println!(" i = {}, mri = {}, mrc = {}, m[i] = {}", i , most_right_index, most_right_center, m[i]);
            if i < most_right_index {
                // 因为i < most_right_index,从对称性可知, j一定也不会越界.
                let j = 2 * most_right_center - i;
                if most_right_index - i > m[j] {
                    m[i] = m[j];
                } else {
                    while (most_right_index + 1) < s.len()
                        && (2 * i >= most_right_index + 1) {
                        if s.as_bytes()[most_right_index + 1] == s.as_bytes()[2 * i - most_right_index - 1] {
                            most_right_index += 1;
                        } else {
                            break
                        }
                    }
                    m[i] = most_right_index - i;
                    most_right_center = i;
                }
            } else {
                most_right_index = i;
                most_right_center = i;
                m[i] = 0;
                let mut tmp = 1;
                while i >= tmp && (i+tmp) < s.len() && s.as_bytes()[i + tmp] == s.as_bytes()[i - tmp] {
                    most_right_index += 1;
                    m[i] = tmp;
                    tmp += 1;
                }
            }
            // println!(" i = {}, mri = {}, mrc = {}, m[i] = {}", i , most_right_index, most_right_center, m[i]);
        }

        let mut c = 0;
        for i in 1..m.len() {
            if m[i] > m[c] {
                c = i;
            }
        }

        // println!("");
        String::from_utf8(s.as_bytes()
            [(c - m[c])..=(c + m[c])]
            .to_vec()).unwrap().replace("#", "")
    }


    /// 预处理字符串, 加#穿插.
    fn pre_deal_palindrome(s: String) -> String {
        let mut new = String::new();
        new.reserve(s.len() * 2);
        for i in s.chars() {
            new.push('#');
            new.push(i);
        }
        new.push('#');
        new
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    pub fn longest_palindromic_test() {
        assert_eq!(Solution::longest_palindrome("bbbbb".to_string()), "bbbbb".to_string());
        assert_eq!(Solution::longest_palindrome("bababd".to_string()), "babab".to_string());
        assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb".to_string());
        assert_eq!(Solution::longest_palindrome("".to_string()), "".to_string());
        assert_eq!(Solution::longest_palindrome("c".to_string()), "c".to_string());
        assert_eq!(Solution::longest_palindrome("ccc".to_string()), "ccc".to_string());
        assert_eq!(Solution::longest_palindrome("cccc".to_string()), "cccc".to_string());
    }

    #[test]
    pub fn longest_palindromic_2_test() {
        assert_eq!(Solution::longest_palindrome_2("bbbbb".to_string()), "bbbbb".to_string());
        assert_eq!(Solution::longest_palindrome_2("bababd".to_string()), "babab".to_string());
        assert_eq!(Solution::longest_palindrome_2("cbbd".to_string()), "bb".to_string());
        assert_eq!(Solution::longest_palindrome_2("".to_string()), "".to_string());
        assert_eq!(Solution::longest_palindrome_2("c".to_string()), "c".to_string());
        assert_eq!(Solution::longest_palindrome_2("ccc".to_string()), "ccc".to_string());
        assert_eq!(Solution::longest_palindrome_2("cccc".to_string()), "cccc".to_string());
    }

    #[test]
    pub fn longest_palindromic_3_test() {
        assert_eq!(Solution::longest_palindrome_3("bbbbb".to_string()), "bbbbb".to_string());
        assert_eq!(Solution::longest_palindrome_3("bababd".to_string()), "babab".to_string());
        assert_eq!(Solution::longest_palindrome_3("cbbd".to_string()), "bb".to_string());
        assert_eq!(Solution::longest_palindrome_3("".to_string()), "".to_string());
        assert_eq!(Solution::longest_palindrome_3("c".to_string()), "c".to_string());
        assert_eq!(Solution::longest_palindrome_3("ccc".to_string()), "ccc".to_string());
        assert_eq!(Solution::longest_palindrome_3("cccc".to_string()), "cccc".to_string());
    }
}
