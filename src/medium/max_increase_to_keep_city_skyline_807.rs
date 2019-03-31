use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/max-increase-to-keep-city-skyline/
    /// 从四边看上去形状都不能变的情况下,最后添加多少格子.
    /// 1. 上,下和左,右看上去都是一样的.
    /// 2. 对于每一个位置的约束, 只要看一下两个方向约束的最小值即可.
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let y_size = grid.len();
        let x_size = grid.get(0).unwrap().len();
        // left and right
        let mut lr_threshold = Vec::new();
        lr_threshold.resize(y_size, 0);
        // bottom and top
        let mut bt_threshold = Vec::new();
        bt_threshold.resize(x_size, 0);
        for j in 0..y_size {
            for i in 0..x_size {
                bt_threshold[i] = bt_threshold[i].max(grid.get(j).unwrap()[i]);
                lr_threshold[j] = lr_threshold[j].max(grid.get(j).unwrap()[i]);
            }
        }

        let mut ret = 0;
        for j in 0..y_size {
            for i in 0..x_size {
                ret += 0.max(bt_threshold[i].min(lr_threshold[j]) - grid.get(j).unwrap()[i]);
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn max_increase_keeping_skyline_test() {
        assert_eq!(Solution::max_increase_keeping_skyline(vec![
            vec![3, 0, 8, 4],
            vec![2, 4, 5, 7],
            vec![9, 2, 6, 3],
            vec![0, 3, 1, 0]
        ]), 35);
        assert_eq!(Solution::max_increase_keeping_skyline(vec![
            vec![3, 0, 8, 4, 5, 67, 43],
            vec![2, 4, 5, 7, 5, 6, 3],
            vec![9, 2, 6, 3, 2, 34, 6],
            vec![0, 3, 1, 0, 3, 76, 3]
        ]), 130);
    }
}

