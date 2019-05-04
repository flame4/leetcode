use crate::Solution;
use crate::listnode::ListNode;

impl Solution {
    /// https://leetcode.com/problems/odd-even-linked-list/
    /// 线性时间, o(1)空间, 把链表换顺序排序，先是偶数位置的数字，再是奇数位置的数字
    /// 尝试直接变更这个线性链表的指针位置在Rust内会遇到所有权冲突的问题，不是很好.
    /// 这里重新开两个链表, 最后接到一起就好了. 时间复杂度和空间复杂度都是ok的.
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() { return None; }
        let mut head = { head };
        let mut odd_list = ListNode::new(0);
        let mut even_list = ListNode::new(0);
        let mut odd_next = &mut odd_list.next;
        let mut even_next = &mut even_list.next;
        let mut offset = 1;
        while let Some(mut n) = head.take() {
            head = n.next.take();
            if (offset & 1) == 1 {
                // offset is even, start from 1.
                even_next.replace(n);
                even_next = &mut even_next.as_mut().unwrap().next;
            } else {
                odd_next.replace(n);
                odd_next = &mut odd_next.as_mut().unwrap().next;
            }
            offset += 1;
        }

//        drop(odd_next);
//        drop(even_next);
//        println!("odd = {:?} \t even = {:?}", odd_list.into_vec(), even_list.into_vec());
        if odd_list.next.is_some() {
            even_next.replace(odd_list.next.take().unwrap());
        }
        even_list.next.take()
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::listnode::ListNode;

    #[test]
    pub fn odd_even_list_test() {
        let v = vec![2, 1, 3, 5, 6, 4, 7];
        let root = ListNode::from_vec(v);
        assert_eq!(Solution::odd_even_list(root).unwrap().show_as_vec(), vec![2, 3, 6, 7, 1, 5, 4]);

        let v = vec![];
        let root = ListNode::from_vec(v);
        assert_eq!(Solution::odd_even_list(root), None);

        let v = vec![1];
        let root = ListNode::from_vec(v);
        assert_eq!(Solution::odd_even_list(root).unwrap().show_as_vec(), vec![1]);
    }
}
