pub mod jewels_and_stones_771;
pub mod n_repeated_element_in_size_2n_array_961;
pub mod unique_email_addresses_929;
pub mod unique_morse_code_words_804;
pub mod squares_of_a_sorted_array_977;
pub mod sort_array_by_parity_905;
pub mod flipping_an_image_832;
pub mod robot_return_to_origin_657;
pub mod di_string_match_942;
pub mod merge_two_binary_trees_617;
pub mod number_of_recent_calls_933;
pub mod array_partition_1_561;
pub mod sum_of_even_number_after_queries_985;
pub mod sort_array_by_parity_ii_922;
pub mod k_closest_points_to_origin_973;
pub mod subdomain_visit_count_811;
pub mod middle_of_the_linked_list_876;


pub struct Solution;


#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}


impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }

    #[allow(dead_code)]
    fn from_vec(array: Vec<i32>) -> Option<Box<Self>> {
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
    fn into_vec(&self) -> Vec<i32> {
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
    fn show(&self) {}
}

use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    // TODO: implement a function to show tree.
    pub fn show() {}
}

pub fn run_jewels_and_stones_771() {
    println!("run jewels_and_stones test!");
    let jewels = String::from("aA");
    let stones = String::from("aAAAbbbb");
    assert_eq!(4, Solution::num_jewels_in_stones(jewels, stones));

    let jewels = String::from("z");
    let stones = String::from("ZZ");
    assert_eq!(0, Solution::num_jewels_in_stones(jewels, stones));

    let jewels = String::from("");
    let stones = String::from("");
    assert_eq!(0, Solution::num_jewels_in_stones(jewels, stones));
}



