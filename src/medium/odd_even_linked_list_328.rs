use crate::Solution;
use crate::listnode::ListNode;

impl Solution {
    /// https://leetcode.com/problems/odd-even-linked-list/
    /// 线性时间, o(1)空间, 把链表换成先是奇数后是偶数的分布.
    /// 因为需要在O(1)时间内完成, 并且不申请新空间. 所以需要维护一个奇数的指针位置.
    /// 新找到一个奇数后, 在这个奇数位置后面放这个节点.
    /// 通过这个问题可以学习一下怎么处理ListNode这种封装格式的指针转换过程.
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = { head };
        // 当前已经访问结束的节点, 也就是说, 每次迭代之后都会去迭代下一个,
        // 并且保证这个是clone的新的, 可以随便操作.
        let mut now_visit_node: *mut Option<Box<ListNode>> = None;
        // 当前的odd节点的tail节点位置.
        let mut odd_position_now: *mut Option<Box<ListNode>> = None;

        // 先初始化第一个节点.
        unsafe {
            if let Some(n) = head.as_ref() {
                if n.val % 2 == 1 {
                    odd_position_now = &head as *mut Option<Box<ListNode>>;
                }
                now_visit_node = &head as *mut Option<Box<ListNode>>;
            }

            while let Some(n) = *now_visit_node {
                if let Some(next) = n.next.as_mut() {
                    if next.val % 2 == 1 {
                        (*n).next = next.next;
                        // 找到一个奇数节点, 把其挂到当前链表后面, 这个位置去掉这个节点.
                        if odd_position_now.is_none() {
                            // odd_position可能没被初始化.
                            (*next).next = head;
                            head = Some(*next);
                        } else {
                            let mut tmp = odd_position_now.unwrap();
                            (*next).next = tmp.next;
                            tmp.next = Some(*next);
                            odd_position_now = tmp.next.as_mut();
                        }
                    } else {
                        // 没找到奇数节点, 往后走.
                        now_visit_node = Some(next)
                    }
                } else {
                    break;
                }
            }
        }
        head
    }
}