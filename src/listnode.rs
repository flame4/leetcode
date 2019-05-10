#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}


impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }

    #[allow(dead_code)]
    pub fn from_vec(array: Vec<i32>) -> Option<Box<Self>> {
        if array.is_empty() { return None; }
        let mut root = Some(Box::new(ListNode::new(0)));
        let mut follow = &mut root;
        let length = array.len();
        for (index, val) in array.into_iter().enumerate() {
            follow.as_mut().unwrap().val = val;
            if index >= length - 1 {
                break;
            }
            follow.as_mut().unwrap().next = Some(Box::new(ListNode::new(0)));
            follow = &mut (follow.as_mut().unwrap().next);
        }
        root
    }

    #[allow(dead_code)]
    pub fn show_as_vec(&self) -> Vec<i32> {
        // TODO implement an Iter
        let mut ret = vec![];
        let mut r = self;
        loop {
            ret.push(r.val);
            if r.next.is_none() { break; }
            r = r.next.as_ref().unwrap().as_ref();
        }
        ret
    }

    /// 生序排序函数, o(nlogn)时间复杂度, 归并排序方法.
    /// 因为快速排序涉及到单链条上的多可变引用修改, 在Rust无法用Safe做到.
    /// 归并排序在拆分节点上可以做到拆开链条变成两条, 所以可以做到.
    #[allow(dead_code)]
    pub fn sort(&self) {}


    /// 升序合并两个排好序的listnode, 归并排序使用.
    pub(self) fn merge_list(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut ret = ListNode::new(0);
        let mut ret_pointer = &mut ret.next;
        loop {
            let mut l1_next = None;
            let mut l2_next = None;
            if let Some(n) = l1.as_mut() {
                l1_next = n.next.take();
            }
            if let Some(n) = l2.as_mut() {
                l2_next = n.next.take();
            }

            match (l1, l2) {
                (Some(mut n1), Some(mut n2)) => {
                    if n1.val < n2.val {
                        ret_pointer.replace(n1);
                        n2.next = l2_next;
                        l2_next = Some(n2);
                    } else {
                        ret_pointer.replace(n2);
                        n1.next = l1_next;
                        l1_next = Some(n1);
                    }
                }
                (Some(n), None) => {
                    ret_pointer.replace(n);
                }
                (None, Some(n)) => {
                    ret_pointer.replace(n);
                }
                (None, None) => break
            }
            l1 = l1_next;
            l2 = l2_next;
            ret_pointer = &mut ret_pointer.as_mut().unwrap().next;
        }
        ret.next
    }
}

#[cfg(test)]
mod tests {
    use crate::listnode::ListNode;

    #[test]
    pub fn listnode_vec_trans_test() {
        let v = vec![3, 2, 42, 2, 5, 2, 3, 4, 5, 2, 3, 4, 5, 2, 3, 4, 1, 1, 3, 1, 3];
        let root = ListNode::from_vec(v.clone());
        assert_eq!(root.unwrap().show_as_vec(), v);
    }

    #[test]
    pub fn merge_list_test() {
        let l1 = ListNode::from_vec(vec![1, 3, 5]);
        let l2 = ListNode::from_vec(vec![2, 4, 6]);
        assert_eq!(ListNode::merge_list(l1, l2).unwrap().show_as_vec(), vec![1, 2, 3, 4, 5, 6]);
    }
}
