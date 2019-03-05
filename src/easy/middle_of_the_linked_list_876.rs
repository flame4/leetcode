use super::Solution;
use super::ListNode;


impl Solution {
    // 对于Rc的clone, 行为是增加其引用技术, 所以直接clone损耗也不大.
    // 对于Box的clone, 文档上写clone为复制堆内存, 但是这样就会导致递归复制, 显然不太合理.
    // 调用clone会导致如下报错:
    // the method `clone` exists but the following trait bounds were not satisfied:
    // `std::option::Option<std::boxed::Box<easy::ListNode>> : std::clone::Clone`
    // 显然, 对Box内部的东西需要自己定义Clone, 因为Box的clone有trait bound, impl<T: Clone> Clone for Box<T>
    // Rc的就没有这个限制.  impl<T: ?Sized> Clone for Rc<T>
    // 正常情况下, 应该使用引用这个最基本指针来进行数据的访问和获取, 但是也要注意技巧才能避免出现一个节点存在引用和移动并存的现象.
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // TODO 把0，1这两种特殊情况也考虑写到一起.
        if head == None { return None; }
        if head.as_ref().unwrap().next.is_none() { return head; }
        let mut follower = head;
        let mut poineer = &(follower.as_ref().unwrap().next);
        loop {
            follower = follower.unwrap().next;
            if poineer.as_ref().unwrap().next.is_none() {
                break;
            } else {
                poineer = &(poineer.as_ref().unwrap().next);
            }
            if poineer.as_ref().unwrap().next.is_none() {
                break;
            } else {
                poineer = &(poineer.as_ref().unwrap().next);
            }
        }
        follower
    }
}



# [cfg(test)]
mod tests {
use super::Solution;
use super::ListNode;

# [test]
pub fn middle_node_1() {
let v = vec ! [1, 2, 3, 4, 5, 6];
let root = ListNode::from_vec(v);
assert_eq! (Solution::middle_node(root).unwrap().into_vec(), vec ! [0]);
}
}