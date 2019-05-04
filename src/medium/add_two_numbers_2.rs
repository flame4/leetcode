use crate::Solution;
use crate::listnode::ListNode;

impl Solution {
    /// https://leetcode.com/problems/add-two-numbers/
    /// 递归的叠加即可, 这里用到了Option的借用, 不断修改这个引用来迭代节点位置.
    /// rust的借用分析还不够强大, 每个分支除了break掉的, 都要把l1,l2赋值一次, 才不会报错moved value.
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut ret = None;
        let mut follower = &mut ret;
        let mut l1 = { l1 };
        let mut l2 = { l2 };
        let mut carry = 0;
        loop {
            match (l1, l2) {
                (None, None) => {
                    if carry != 0 {
                        *follower = Some(Box::new(ListNode::new(carry)));
                    }
                    break;
                }
                (Some(lb1), None) => {
                    let new_val = (lb1.val + carry) % 10;
                    carry = (lb1.val + carry) / 10;
                    *follower = Some(Box::new(ListNode::new(new_val)));
                    l1 = lb1.next;
                    l2 = None;
                }
                (None, Some(lb2)) => {
                    let new_val = (lb2.val + carry) % 10;
                    carry = (lb2.val + carry) / 10;
                    *follower = Some(Box::new(ListNode::new(new_val)));
                    l1 = None;
                    l2 = lb2.next;
                }
                (Some(lb1), Some(lb2)) => {
                    let new_val = (lb1.val + lb2.val + carry) % 10;
                    carry = (lb1.val + lb2.val + carry) / 10;
                    *follower = Some(Box::new(ListNode::new(new_val)));
                    l1 = lb1.next;
                    l2 = lb2.next;
                }
            }
            follower = &mut (follower.as_mut().unwrap().next)
        }
        ret
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;
    use crate::listnode::ListNode;

    #[test]
    pub fn add_two_numbers_test() {
        assert_eq!(ListNode::from_vec(vec![7, 0, 8]).unwrap().show_as_vec(),
                   Solution::add_two_numbers(
                       ListNode::from_vec(vec![2, 4, 3]),
                       ListNode::from_vec(vec![5, 6, 4])).unwrap().show_as_vec()
        );
        assert_eq!(ListNode::from_vec(vec![3, 0, 0, 1]).unwrap().show_as_vec(),
                   Solution::add_two_numbers(
                       ListNode::from_vec(vec![4, 0, 1]),
                       ListNode::from_vec(vec![9, 9, 8])).unwrap().show_as_vec()
        );
    }
}