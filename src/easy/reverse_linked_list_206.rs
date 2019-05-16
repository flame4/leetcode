use crate::Solution;
use crate::listnode::ListNode;

impl Solution {
    /// https://leetcode.com/problems/reverse-linked-list/
    /// 单链表反转问题.
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() { return None; }
        let mut head = { head };
        let mut ret = None;
        while let Some(mut n) = head {
            head = n.next.take();
            n.next = ret;
            ret = Some(n);
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::listnode::ListNode;

    #[test]
    pub fn reverse_list_test() {
        assert_eq!(Solution::reverse_list(ListNode::from_vec(vec![1,2,3,4,5])).unwrap().show_as_vec(), vec![5,4,3,2,1]);
    }
}


