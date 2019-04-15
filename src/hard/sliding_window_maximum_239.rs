use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/sliding-window-maximum/
    /// 头条算法题.
    /// 滑动窗口k, 不断输出最大值. 唯一的问题是要解决当最大值滑出去的话该怎么办.
    /// 简单的想一下,假设我们目前窗口内, a[i] ~ a[i+k], 最大数字是 a[j].  i<j<i+k.
    /// 那么我们知道j之前的数字是没有必要比较的, 是废物数据, 随着窗口从i滑到j, 会出现以下这么几种情况:
    /// 1. 进来了一个比j大的. 那么我们自然可以把j换为新的j.然后继续看.
    /// 2. 直到j要滑出去, 都没遇到比a[j]大的值. 那么j滑出去的时候, 我们其实要找一个在a[j] 到 a[j+k]之间
    /// 比a[j]小的, 最大的一个值即可. 那么我们在不断遍历过程中, 这点是可以做到的.
    /// 但是我们又不能单纯维护一个值. 比如比a[j]小的最大那个是a[m], 但是我们把a[j]推出去的时候, a[m]就会把
    /// a[j]替换下来, 但是我们还需要知道谁来替换a[m], 这个操作, 我们可以维护一个双端数组来做到.
    /// 同时, 我们不需要记录当前最大位置, 定义数组内的第一个元素就是最大值即可.
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::vec_deque::VecDeque;
        // 记录当前最右边访问到的index位置.
        let mut index = 0;
        // 存的一定是下标, 因为要知道什么时候被弹出去.
        let mut queue = VecDeque::new();

        // 初始化数组.
        for i in 0..nums.len().min(k as usize) {
            if queue.is_empty() {
                queue.push_back(i);
            } else {
                // 这个等号比较重要.
                while !queue.is_empty() && nums[*queue.back().unwrap()] <= nums[i] {
                    queue.pop_back();
                }
                queue.push_back(i);
            }
            index = i;
        }

        let mut ret = Vec::new();
        if let Some(v) = queue.front() {
            ret.push(nums[*v]);
        }

        for i in (index + 1)..nums.len() {
            // println!("i = {}, {:?}", i, queue);
            // i代表窗口开始滑动的下一个位置.
            // 窗口肯定是有取值的.
            if *queue.front().unwrap() == i - (k as usize) {
                // 表示最大的要被pop出去了.
                queue.pop_front();
            }
            // 不管最大的是否出去了, 现在头上的就是当前还在窗口内的最大的了
            while !queue.is_empty() && nums[*queue.back().unwrap()] < nums[i] {
                queue.pop_back();
            }
            queue.push_back(i);
            ret.push(nums[*queue.front().unwrap()]);
            // println!("i = {}, {:?}", i, queue);
        }
        ret
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    pub fn sliding_window_maximum_test() {
        assert_eq!(Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3), vec![3, 3, 5, 5, 6, 7]);
        assert_eq!(Solution::max_sliding_window(vec![], 3), vec![]);
        assert_eq!(Solution::max_sliding_window(vec![2,4,5,6,7,4], 8), vec![7]);
        assert_eq!(Solution::max_sliding_window(vec![234,32,42,4,2353,3,34,2,34,-8,-456,-4,345,6363,-345,450,34,4,5], 5),
                   vec![2353,2353,2353,2353,2353,34,34,34,345,6363,6363,6363,6363,6363,450]);
    }
}


