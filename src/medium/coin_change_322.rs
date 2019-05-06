use crate::Solution;


impl Solution {
    /// https://leetcode.com/problems/coin-change/
    /// 换零钱问题, 给定一个零钱集合和总钱数, 返回最少的可以构成这个钱的硬币集合数字, 没有返回-1.
    /// 典型的动态规划问题.
    /// 需要amount这么大的搜索空间, 每个钱都需要根据硬币数字搜索前向空间, O(C * A)的复杂度.
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut m : Vec<u32> = Vec::new();
        m.resize(amount as usize + 1, std::u32::MAX);
        m[0] = 0;

        for money in 0..=amount {
            for coin in &coins {
                if *coin > money {
                    continue
                } else if *coin == money {
                    m[money as usize] = 1;
                } else {
                    if m[(money - *coin) as usize] < std::u32::MAX {
                        m[money as usize] = m[money as usize].min(1 + m[(money - *coin) as usize])
                    }
                }
            }
        }
        // u32的最大值就是i32的-1
        m[amount as usize] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn coin_change_test() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
        assert_eq!(Solution::coin_change(vec![2], 3), -1);
        assert_eq!(Solution::coin_change(vec![1], 0), 0);
    }
}
