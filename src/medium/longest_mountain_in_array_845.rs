use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/longest-mountain-in-array/
    /// 山峰定义为先严格连续递增后严格连续递减的序列.
    /// 找到最长山峰，只需要顺序扫描即可.
    /// 这个问题可以练习怎么样统一思路, 从各种分支判断条件里面抽取出一种简单的模式.
    /// 要注意一点, 这个山峰必须有上升部分和下降部分, 不能只有其中一部分.
    pub fn longest_mountain(a: Vec<i32>) -> i32 {
        if a.is_empty() { return 0; }
        let mut ret = 0;
        // 意思为, 截止到当前位置(包含),山峰长度有多少.
        let mut m_len = 1;
        // true 表示当前要寻找的是递增部分.
        // false表示当前要寻找的是递减部分.
        let mut direct = true;

        for i in 0..(a.len() - 1) {
            if a[i] == a[i + 1] {
                // 查找结束. a[i]在进入循环已经加入m_len,看定义.
                ret = ret.max(m_len);
                // 根据定义, 到a[i+1]的时候, 它是第一个起点了.
                m_len = 1;
                direct = true;
            } else if a[i] > a[i + 1] {
                // 要么是转折点, 要么是下游位置, 山峰都可以继续.
                if direct {
                    // 转折点的判断必须加一条, 判断其是不是有了上升的部分.
                    if m_len > 1 {
                        m_len += 1;
                        direct = false;
                    } else {
                        // 不构成山峰, 从头开始
                        m_len = 1;
                        direct = true;
                    }
                } else {
                    m_len += 1;
                }
            } else {
                // 上升阶段.
                if direct {
                    m_len += 1;
                } else {
                    // 意味着这个山峰的终结, 同时a[i]和a[i+1]都可以作为下一个山峰的起点
                    ret = ret.max(m_len);
                    m_len = 2;
                    direct = true;
                }
            }
        }
        // 最后一个下降阶段可能没同步结果，这里补上。
        if !direct {
            ret = ret.max(m_len);
        }

        // 条件定义mountain长度 >= 3
        if ret < 3 {
            ret = 0
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    pub fn longest_mountain_test() {
        assert_eq!(Solution::longest_mountain(vec![2, 1, 4, 7, 3, 2, 5]), 5);
        assert_eq!(Solution::longest_mountain(vec![2, 2, 2]), 0);
        assert_eq!(Solution::longest_mountain(vec![2,3,4,5,6,4,6,3,4,6,5,6,7,8,9,10,11,9,7,4,5,2,3,5,6]), 10);
        assert_eq!(Solution::longest_mountain(vec![]), 0);
        assert_eq!(Solution::longest_mountain(vec![1]), 0);
        assert_eq!(Solution::longest_mountain(vec![0,1,2,3,4,5,4,3,2,1,0]), 11);
        assert_eq!(Solution::longest_mountain(vec![0,1,2,3,4,5,6,7,8,9]), 0);
        assert_eq!(Solution::longest_mountain(vec![9,8,7,6,5,4,3,2,1,0]), 0);
    }
}


