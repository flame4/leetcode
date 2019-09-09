use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/monotone-increasing-digits/
    /// 给定一个数字, 返回小于等于它的最大数字, 该数字需要满足位置从左到右非递减, 简称满足 MID 条件.
    /// naive 的方法可以从 n 往下找, 设 n 的位数为 L, 预期复杂度为 O(nL)
    /// 我们希望找到一个复杂度为 O(L) 的算法, 思路如下:
    /// 我们多举几个例子可以看到, 从右往左看, 实际生效规则可以总结为,
    /// 当我们遇到一位, 它左边比它要大, 那么就把它左边那位-1, 它自己和右边所有数字变成 9.
    /// 这可以用一个栈来实现, 只需要注意数字结束时候的边界条件即可.
    pub fn monotone_increasing_digits(n: i32) -> i32 {
        let mut n = n;
        let mut stack = Vec::new();
        while n > 0 {
            let digits = n % 10;
            stack.push(digits);
            n = n / 10;
            if n > 0 {
                // 还有高位, 执行上述逻辑.
                let left_digits = n % 10;
                if left_digits > digits {
                    n = n - 1;
                    // 可以用 pos 记录来有优化一下, 懒得写了.
                    stack.iter_mut().for_each(|x| *x = 9)
                }
            }
        }
        stack.iter().rev().fold(0, |x, y| x * 10 + y)
    }
}


#[cfg(test)]
mod tests {

    use crate::Solution;

    #[test]
    pub fn monotone_increasing_digits_test() {
        assert_eq!(Solution::monotone_increasing_digits(10), 9);
        assert_eq!(Solution::monotone_increasing_digits(1234), 1234);
        assert_eq!(Solution::monotone_increasing_digits(332), 299);
        assert_eq!(Solution::monotone_increasing_digits(0), 0);
        assert_eq!(Solution::monotone_increasing_digits(654321), 599999);
        assert_eq!(Solution::monotone_increasing_digits(36759), 36699);
        assert_eq!(Solution::monotone_increasing_digits(36259), 35999);
        assert_eq!(Solution::monotone_increasing_digits(36559), 35999);
    }
}
