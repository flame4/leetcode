use super::Solution;


impl Solution {
    // 每组两个数字取最小值，求所有组合里加和最大的方案.
    // 思路: 无论什么分配, 最小的那个数字肯定要贡献自己
    // 他能带走一个数字, 那么带走谁是最好的呢? 当然是第二小的数字, 这样第二小的数字就不会拖低平均数了.

    // v1 直接排序. faster than 10%. 16ms.
    pub fn array_pair_sum_v1(nums: Vec<i32>) -> i32 {
        let mut sort_nums = Vec::from(nums);
        sort_nums.sort();
        let mut ret = 0;
        for (index, value) in sort_nums.into_iter().enumerate() {
            if index % 2 == 0 {
                ret += value;
            }
        }
        ret
    }

    // v2 优化为内部迭代器. From network. 12ms.
    pub fn array_pair_sum_v2(nums: Vec<i32>) -> i32 {
        let mut sort_nums = nums.clone();
        sort_nums.sort();
        sort_nums.chunks_exact(2).map(|x| x[0]).sum()
    }

    // v3 使用enumerate迭代器. 8ms.
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums.clone();
        nums.sort();
        nums.into_iter().enumerate().filter_map(|x| {
            if x.0 % 2 == 0 {
                Some(x.1)
            } else {
                None
            }
        }).sum()
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn array_partition_i_1() {
        assert_eq!(Solution::array_pair_sum(vec![1,4,3,2]), 4);
    }
}