use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/gas-station/
    /// 给两个数组, gas[i]表示在i这个加油站可以加多少单位油, cost[i]表示从i走到i+1(环形)需要消耗多少油.
    /// 问是不是可以从一个点开始走, 能走一圈.
    /// 首先, 要想清楚一个结论: 如果总油量>=总消耗, 总有一个点可以是合法的起点, 如果总油量 < 总消耗, 没有合法起点.
    /// 因为总可以把一条多节点路径的节点合并来计算总体的得失.
    ///
    /// 所以如果是单纯的判断是否可行, 求一个和即可.
    /// 但是要顺便把起点求出来, 假设我们随意指定一个起点i往前走, 走到了j, 并且油不够走到了j+1
    /// 我们可以说, i到j的任何一个点都不能作为起点. 因为i到j的每一个点肯定油量剩余都是正的, 不然走不到j.
    /// 那既然加起来都不够, 去掉任何一个点都不够. 这样把起点设置为j+1.
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut total = 0;
        let mut pos = 0;
        for i in 0..gas.len() {
            sum += gas[i] - cost[i];
            total += gas[i] - cost[i];
            if sum < 0 {
                pos = (i+1) as i32;
                sum = 0
            }
        }

        if total < 0 {
            -1
        } else {
            pos
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    pub fn can_complete_circuit_test() {
        assert_eq!(Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]), 3);
        assert_eq!(Solution::can_complete_circuit(vec![0, 0, 0, 0, 10], vec![1, 1, 1, 1, 3]), 4);
    }
}

