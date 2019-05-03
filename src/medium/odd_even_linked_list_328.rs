use crate::Solution;
use crate::listnode::ListNode;
use std::mem;

impl Solution {
    /// https://leetcode.com/problems/odd-even-linked-list/
    /// 线性时间, o(1)空间, 把链表换成先是奇数后是偶数的分布.
    /// 因为需要在O(1)时间内完成, 并且不申请新空间. 所以需要维护一个奇数的引用位置.
    /// 新找到一个奇数后, 在这个奇数位置后面放这个节点.
    /// 首先，我们不挪动节点，而是交换节点的值.
    /// 对于奇数的尾部引用, 我们不需要维护可变引用, 因为我们每次修改的都是这个节点的下一个节点.
    /// 对于当前访问的指针, 我们需要维护可变指针, 其可能变化.
    /// 这样实现的问题是, 我们不知道什么时候才能找到第一个奇数值, 但是我们从header进去的时候只能拿到一个mut的引用值，
    /// 导致我们不能延时赋值给head. 否则会出现可变/不可变借用同时存在的场景.
    /// 所以我们必须先走一步, 找到第一个奇数值, 把head给初始化掉.
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() { return None; }
        let mut head = { head }.unwrap();
        // 当前即将访问的节点.
        let mut now_visit_node: Option<&mut Box<ListNode>>;
        // 当前的odd节点的tail节点位置.
        let mut odd_position_now: Option<&Box<ListNode>>;

        // 先初始化第一个节点.

        let mut tmp = head.next.as_mut();
        let mut has_odd = false;
        loop {
            if let Some(n) = tmp {
                tmp = n.next.as_mut();
                if n.val % 2 == 1 {
                    mem::swap(&mut head.val, &mut n.val);
                    has_odd = true;
                    break;
                }
            } else {
                break;
            }
        }
        if !has_odd {
            return Some(head);
        }
        now_visit_node = tmp;
        odd_position_now = Some(&head);


        // 接下来就可以循环查找.
        loop {
            let mut tmp = now_visit_node.unwrap();
            now_visit_node = tmp.next.as_mut();
            if tmp.val % 2 == 1 {
                // 可能还没找到一个odd开头的节点. 但是因为前面已经借用了header的头，这里用unsafe创建一个最小不安全区域重新修改一下.

                let mut odd_next = odd_position_now.unwrap().next.as_mut().unwrap();
                mem::swap(&mut tmp.val, &mut odd_next.val);
            }

            odd_position_now = odd_position_now.unwrap().next.as_ref();
            if now_visit_node.is_none() {
                break;
            }
        }
        Some(head)
//        unsafe {
//            // 先初始化第一个节点.
//            if let Some(n) = head_ptr.as_mut() {
//                if n % 2 == 1 {
//                    odd_position_now = &mut head_ptr.as_mut() as *mut Option<&mut Box<ListNode>>;
//                }
//                now_visit_node = &mut head.as_mut() as *mut Option<&mut Box<ListNode>>;
//            }
//
//            while let Some(n) = *now_visit_node {
//                if let Some(next) = n.next.as_mut() {
//                    if next.val % 2 == 1 {
//                        (*n).next = next.next;
//                        // 找到一个奇数节点, 把其挂到当前链表后面, 这个位置去掉这个节点.
//                        if (*odd_position_now).is_none() {
//                            // odd_position可能没被初始化.
//                            (*next).next = head;
//                            head = Some(next);
//                        } else {
//                            let mut tmp = odd_position_now.unwrap();
//                            (*next).next = tmp.next;
//                            tmp.next = Some(*next);
//                            odd_position_now = tmp.next.as_mut();
//                        }
//                    } else {
//                        // 没找到奇数节点, 往后走.
//                        now_visit_node = &mut n.next.as_mut() as *mut Option<&mut Box<ListNode>>;
//                    }
//                } else {
//                    break;
//                }
//            }
//        }
    }
}