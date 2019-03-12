use std::rc::Rc;
use std::cell::RefCell;
use crate::Solution;
use crate::treenode::TreeNode;


impl Solution {
    /// https://leetcode.com/problems/maximum-binary-tree/
    /// 最简单的思路是递归的构造树，找到最大值，然后分别递归构造左子树和右子树即可.
    /// 时间复杂度主要查找最大值上, 为O(N*N)
    /// 最优解法期望达到更好的时间复杂度. 思路如下:
    /// 我们考虑前k个数字已经组成了一棵符合条件的树, 根节点对应的位置为 i <= k.
    /// 那么考虑a[k+1] 和 a[i] 的大小关系.
    /// 1. a[i] > a[k+1], 因为要求根节点右边的树一定比根节点小, 符合题意, 令a[k+1]和a[i].right比较, 如果right为空, a[k+1]就放在这里.
    /// 2. a[i] < a[k+1], 因为a[i]已经是前面最大的了, 那么需要做的就是a[k+1] 成为新的根节点, a[i]变为其左节点.
    /// 3. 递归的角度来看 i不一定是根节点，也可以是从1进入的子树的节点, 也符合这两个递归条件, 因为a[k+1]一定是当前情况下最右边的那个节点.
    /// 由以上规则可以递归的构造出这个树来, 当然, 对Rust来说, 怎么修改这个值, 可能要操作一下.
    /// 但是这个算法的最差时间复杂度也是O(N^2)的.
    /// TODO: 打脸的是, 这个写法没有直接递归快...... 我需要靠清楚这是为什么...
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() { return None; }
        let root = Rc::new(RefCell::new(TreeNode::new(nums[0])));
        for i in &nums[1..] {
            Solution::insert_rightmost_num_in_node(root.clone(), *i);
        }
        Some(root)
    }

    // 在条件2的左旋转条件下, 我们也许需要修改这个节点的父亲节点的指针.
    // 因为我们不可能把rightmost放到左子树, 也就是说这个函数本身可能递归很多层,
    // 但是每一层我们都一定是因为它父亲比rightmost大, 我们进入了右子树.
    // 所以我们也只需要修改右子树即可.
    fn insert_rightmost_num_in_node(node: Rc<RefCell<TreeNode>>, rightmost: i32) {
        let node_val = node.borrow().val;
        if node_val > rightmost {
            if node.borrow().right.is_none() {
                node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(rightmost))));
            } else {
                Solution::insert_rightmost_num_in_node(node.borrow().right.clone().unwrap(), rightmost);
            }
        } else {
            let left_son_of_origin_node = node.borrow().left.clone();
            let right_son_of_origin_node = node.borrow().right.clone();
            let origin_node_val = node.borrow().val;
            let new_left = Rc::new(RefCell::new(TreeNode::new(origin_node_val)));
            node.borrow_mut().val = rightmost;
            node.borrow_mut().right = None;
            new_left.borrow_mut().left = left_son_of_origin_node;
            new_left.borrow_mut().right = right_son_of_origin_node;
            node.borrow_mut().left = Some(new_left);
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;
    use crate::treenode::TreeNode;
    #[test]
    pub fn sum_even_after_queries_1() {
        let root1 = Solution::construct_maximum_binary_tree(vec![3, 2, 1, 6, 0, 5]);
        let root2 = TreeNode::from_layer_vec(vec![
            Some(6),
            Some(3),
            Some(5),
            None,
            Some(2),
            Some(0),
            None,
            None,
            Some(1)
        ]);
        assert_eq!(root1.clone().unwrap().borrow().post_order(), root2.clone().unwrap().borrow().post_order());
        assert_eq!(root1.clone().unwrap().borrow().pre_order(), root2.clone().unwrap().borrow().pre_order());
    }
}

