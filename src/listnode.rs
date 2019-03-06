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
    pub fn into_vec(&self) -> Vec<i32> {
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

    #[allow(dead_code)]
    pub fn show(&self) {}
}
