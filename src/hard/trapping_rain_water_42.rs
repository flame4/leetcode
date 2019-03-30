use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/trapping-rain-water/
    /// 我们期望在O(n)的时间内完成计算, 简单的画个图, 如下描述这个问题.
    /// 1. 每次一个水坑的构成是由一个非递增连续序列加上一个非递减连续序列组成的.
    /// 2. 一个坑的右边的顶点能提供的水位取决于左边所有出现的水坑里面, 从右往左看的第一个比它高的边缘. 参考下面的图.
    /// 假设a,b,c都是右边, a,b里面有很多坑组成, b,c相邻(就是说是一个坑), c能存多少水, 看到a就行了.
    /// 3. c和a之间增量的水, 只是c比b多出来的高度到a的部分补上水, 加上bc坑之间的水即可.
    /// bc坑之间的水位可以在往后遍历的过程中就计算好,而a,b,c等位置可以用一个栈保存起位置.
    /// 每次找到一个新的顶点C, 要做的事情是弹出所有不高于c的点, 并把c放入.
    ///  a
    /// |-|                 c
    /// | |                |-|
    /// | |       b        | |
    /// | |      |-|       | |
    /// | | ...  | |       | |
    /// | |      | |       | |
    /// |_|______|_|_______|_|____________
    ///
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut stack = Vec::new();
        let mut ret = 0;
        let mut index = 0;

        // 找到第一个>0的起点, 并且要保证是其后一个点比它小, 才能构成一个坑.
        while index + 1 < height.len() {
            if height[index] == 0 || height[index + 1] >= height[index] {
                index += 1;
            } else {
                stack.push(index);
                break;
            }
        }

        // 每次找到一个坑的位置.
        // 每次的搜索规则为找到最右边, 下一个一定是严格小于的位置, 这样能保证找坑逻辑简单, 统一.
        while index < height.len() {
            // 先找降部分.
            let begin = index;
            let mut end = index;
            while index + 1 < height.len() && height[index + 1] <= height[index] {
                index += 1;
            }
            // 找升部分
            while index + 1 < height.len() && height[index + 1] >= height[index] {
                index += 1;
                end = index;
            }
            // 再找不到一个坑了, 说明到最后了.
            if end == begin { break; }

            println!("begin = {}, end = {}, ret = {}, stack = {:#?}", begin, end, ret, stack);

            // 计算这个坑可以装多少水.
            let min_height = height[begin].min(height[end]);
            for i in begin..=end {
                if height[i] < min_height {
                    ret += min_height - height[i];
                }
            }
            println!("ret = {}, stack = {:#?}", ret, stack);


            // 计算需要填补的部分.
            while !stack.is_empty() {
                let last1 = stack.pop().unwrap();
                let last2 = stack.pop().unwrap_or(end + 1);
                if height[last1] >= height[end] {
                    ret += (end - last1 - 1) as i32 * (height[last1] - height[end]);
                    if (last2 < end) { stack.push(last2); }
                    stack.push(last1);
                    break;
                } else if last2 < end {
                    if height[last2] < height[end] {
                        ret += (end - last2 - 1) as i32 * (height[last2] - height[last1]);
                        stack.push(last2);
                    } else {
                        ret += (end - last2 - 1) as i32 * (height[last2] - height[end]);
                        stack.push(last2);
                        break;
                    }

                }
            }
            stack.push(end);

            println!("ret = {}, stack = {:#?}", ret, stack);
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn trap_test() {
        //assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
//        assert_eq!(Solution::trap(vec![]), 0);
//        assert_eq!(Solution::trap(vec![3, 1, 2, 1, 2, 1, 3]), 8);
//        assert_eq!(Solution::trap(vec![2, 0, 0, 2]), 4);
//        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
//        assert_eq!(Solution::trap(vec![3, 2, 4, 5, 2, 4, 6, 7, 3, 45, 7, 87, 3, 4, 34, 6, 3, 3, 6, 54, 2, 34, 6, 4, 6, 7, 4]), 403);
//        assert_eq!(Solution::trap(vec![87, 3, 4, 34, 6, 3, 3, 6, 54]), 319);
//        assert_eq!(Solution::trap(vec![1, 2, 3, 4, 5, 6]), 0);
    }
}
