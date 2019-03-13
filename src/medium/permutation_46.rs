use crate::Solution;


impl Solution {
    /// https://leetcode.com/problems/permutations/
    /// 递归的实现, 对于一个a[i] 和 a[i+1]向后的子序列, 实现所有permutation的规则
    /// 是把a[i] 和 a[i+1] 的每一个元素换个位置.
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = { nums };
        let mut ret = Vec::new();
        if nums.is_empty() { return ret; }
        Solution::recursive_permute(&mut nums, 0, &mut ret);
        ret
    }

    /// pos表示当前的视角观察第pos个位置, 也就是上面思路内的i.
    fn recursive_permute(nums: &mut Vec<i32>, pos: usize, ret: &mut Vec<Vec<i32>>) {
        if pos < nums.len() - 1 {
            Solution::recursive_permute(nums, pos+1, ret);
            for i in pos+1..nums.len() {
                nums.swap(i, pos);
                Solution::recursive_permute(nums, pos+1, ret);
                nums.swap(i, pos);
            }
        } else {
            ret.push(nums.clone());
        }
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn permute_test() {
        let v = vec![1, 2, 3];
        let a = Solution::permute(v);
        let b = vec![
            vec![1,2,3],
            vec![1,3,2],
            vec![2,1,3],
            vec![2,3,1],
            vec![3,2,1],
            vec![3,1,2]
        ];
        assert_eq!(a, b);
    }
}
