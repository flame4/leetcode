use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/unique-paths/
    /// 求问二维方格从左上到右下的走法一共有多少种.
    /// 典型的动态规划路径问题, 一层一层往下走, 可以使用滚动数组.
    /// 定义p[i][j]为从0, 0走到i, j一共有多少种走法.
    /// 状态公式为.
    /// p[i][j] = p[i-1][j] + p[i][j-1]
    /// 注意输入m是长, n是宽
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut p = Vec::new();
        p.resize(n as usize, 1);
        for _ in 1..m {
            for index in 1..n {
                let i = index as usize;
                p[i] = p[i] + p[i-1];
            }
        }

        p[(n-1) as usize]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    pub fn unique_paths_test() {
        assert_eq!(Solution::unique_paths(3, 2), 3);
        assert_eq!(Solution::unique_paths(5, 5), 70);
        assert_eq!(Solution::unique_paths(10, 10), 48620);
        assert_eq!(Solution::unique_paths(15, 15), 40116600);
    }
}
