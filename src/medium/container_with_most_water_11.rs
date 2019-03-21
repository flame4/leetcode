use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/container-with-most-water/
    /// 在一堆顺序安放的长度棍子内找到最大的蓄水池容量, 等价于一系列高楼的最大面积问题.
    /// 算法是 O(n)的.
    /// 首先, 因为是计算面积的, 所以和两个端子之间的距离也有关系, 就是说我们在考虑问题的时候,
    /// 不可缺少的信息是考虑这个端子和前面所有端子之间的距离关系, 我们在迭代过程中必须要考虑怎么样利用
    /// 前面的数据给后面提供信息, 以达到O(n)的算法.
    /// 怎么提供? 长度是一个纬度, 宽度是另一个纬度, 如果我们从两端向中间迭代宽度, 那么我们就控制了
    /// 宽度逐渐减少, 只需要比较内部的杆子是不是可以提供更高的长度即可.
    pub fn max_area(height: Vec<i32>) -> i32 {
        if height.is_empty() { return 0; }
        let mut begin = 0;
        let mut end = height.len() - 1;
        let mut ret = 0;
        while begin < end {
            match height[begin] > height[end] {
                true => {
                    ret = ret.max(height[end] * (end - begin) as i32);
                    end -= 1;
                }
                false => {
                    ret = ret.max(height[begin] * (end - begin) as i32);
                    begin += 1;
                }
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    pub fn max_area_test() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::max_area(vec![]), 0);
        assert_eq!(Solution::max_area(vec![1]), 0);
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7, 23, 2, 5, 3, 45, 6, 3, 4, 2, 5]), 96);
        assert_eq!(Solution::max_area(vec![2, 3, 5, 3, 6, 5, 7, 4, 45, 7]), 35);
        assert_eq!(Solution::max_area(vec![2, 3, 5, 4, 76, 7, 4, 5, 6, 7]), 35);
    }
}
