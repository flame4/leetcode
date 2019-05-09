use crate::Solution;
use crate::listnode::ListNode;

impl Solution {
    /// https://leetcode.com/problems/remove-nth-node-from-end-of-list/
    /// 删除从末尾数的第n个节点.
    /// 1. 链表的思想使用拆开链表组合新链表的思想来避免所有权问题.
    /// 2. 维护两个指针, 距离为n即可.
    /// 比较坑的一点是, Rust识别指针的时候只看逻辑, 不看跳转, 也就是说, 如果我的前向指针是引用,
    /// 后续指针也是引用, 因为两个引用都是从head出去的, 即使真的不会指到同一个上面，也是会认定规则的.
    /// 所以怎么处理这个还是一个问题.
    /// 目前没看到合适的解法,
    /// 要么是先遍历一次求得链表长度, 要么是新分配内存来做.
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let v = head.unwrap().show_as_vec();
        let mut new_vec = Vec::new();
        let size = v.len();
        for (index, val) in v.into_iter().enumerate() {
            if index == size - n as usize {
                continue;
            }
            new_vec.push(val);
        }
        ListNode::from_vec(new_vec)
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;
    use crate::listnode::ListNode;

    #[test]
    pub fn remove_nth_from_end_test() {
        assert_eq!(Solution::remove_nth_from_end(ListNode::from_vec(vec![1, 2, 3, 4, 5]), 2).unwrap().show_as_vec(), vec![1, 2, 3, 5]);
    }
}
