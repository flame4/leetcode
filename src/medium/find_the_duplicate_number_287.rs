use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/find-the-duplicate-number/
    /// 一个n+1长度的数组, 只有一个duplicate数字, 找到它.
    /// 空间复杂度 o(1)
    /// 时间复杂度, 小于o(n^2).
    /// 最简单的方式, 排序.
    /// 关心的是有没有o(n)时间内的解决方法.
    /// Cycle Detection 算法可以满足, 核心思想叫做快慢指针, 在链表排序中也有用到.
    /// 这个例子中, 核心思路是1-n的数据分布在1-n+1的坑里.
    /// 那假设是n个数据分布在n个坑里，利用下表的跳转一定能形成首位环路.
    /// 但是因为里面混入了一个dup数据, 相当于这个环被在中间打断, 接回到了前面的某处.
    /// 我们需要找到这个环的开始点, 其index就是那个数字.
    /// 对于传统的环形指针，解法如下:
    /// 1. 快慢指针从链表开头开始, 快的每次走2, 慢的每次走1.
    /// 2. 当快慢指针相遇的时候, 假设慢指针走了n, 那么快指针就走了2n.
    /// 3. 这时候快指针再从头开始开始走, 但是每次走1, 那么假设慢指针再走n, 快指针就会走n, 两者相遇的地方还是第一次相遇的地方.
    /// 4. 因为第二次相遇是在环里, 那么相遇之前的路径都是一样的, 相遇点就在环路的入口.
    /// 对于这个问题来说, 还有个问题, 链表起点在哪儿！
    /// 这个问题的一个变形理解的重要点在于. 我们不需要把所有数字找到一条链, 我们可以找链条的一部分, 只要这部分
    /// 包含完整的环路就行. 那么这个链条的起点一定可以从 nums[0] 开始. 因为没有一个值指向0(范围是1-n).
    /// 顺便需要理解一点是, 有可能某些数字根本就不在链条上, 比如下标等于值的那些.
    ///
    ///
    /// 还有一种弱智想法...  既然n+1个数 = 1 + 2 + ... + n + x
    /// 那求个和 - 1 ~ n 不就好了...
    /// 错误的点在于, 1 ~ n不一定每个数字都有, 因为duplicate的那个可能有多份.
    //    pub fn find_duplicate_2(nums: Vec<i32>) -> i32 {
    //        let n = nums.len() as i32;
    //        nums.iter().fold(0, | a, b| a + *b) - n * (n - 1) / 2
    //    }
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        // fast, slow都表示当前指向节点的index
        let mut fast = 0;
        let mut slow = 0;
        loop {
            slow = nums[slow as usize];
            fast = nums[nums[fast as usize] as usize];
            if slow == fast { break; }
        }
        fast = 0;
        while slow != fast {
            slow = nums[slow as usize];
            fast = nums[fast as usize];
        }
        slow
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn find_duplicate_test() {
        assert_eq!(Solution::find_duplicate(vec![1, 3, 4, 2, 2]), 2);
        assert_eq!(Solution::find_duplicate(vec![3, 1, 3, 4, 2]), 3);
        assert_eq!(Solution::find_duplicate(vec![3, 1, 3, 4, 2, 3]), 3);
        assert_eq!(Solution::find_duplicate(vec![2, 2, 2, 2, 2]), 2);
    }
}
