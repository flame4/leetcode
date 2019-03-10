use crate::Solution;


impl Solution {
    /// https://leetcode.com/problems/delete-and-earn/
    /// 一个重要的限制是, 每个数字最大为10000.
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        // 每个数字可以提供的分数.
        let mut points = Vec::new();
        points.resize(10001, 0);
        nums.iter().for_each(|i| {
            points[*i as usize] += *i;
        });

        let mut ret = 0;
        // win[i]代表前i个数字的抽取结果可以最多提供多少分.
        // win[i].0 代表第i个数字被选择，1代表没被选择.
        let mut win = Vec::new();
        win.resize(10001, (0,0));
        for i in 1..=10000 {
            win[i].0 = win[i-1].1 + points[i as usize];
            win[i].1 = win[i-1].0.max(win[i-1].1);
            ret = ret.max(win[i].0).max(win[i].1);
        }
        ret
    }
}


#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    pub fn delete_and_earn_test() {
        assert_eq!(Solution::delete_and_earn(vec![3, 4, 2]), 6);
        assert_eq!(Solution::delete_and_earn(vec![2, 2, 3, 3, 3, 4]), 9);
        assert_eq!(Solution::delete_and_earn(vec![9,4,2,10,7,8,8,1,9,4,3,6,7,3,6,3,6,7,3,2,5,7,2,5,3,6,7,2,3,5,7,3,2,5,6,3,5,3,5]), 118);
    }
}

