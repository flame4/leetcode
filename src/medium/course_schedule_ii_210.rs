use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/course-schedule-ii/
    /// 和207几乎一样, 就是把结果存一下而已.
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::vec_deque::VecDeque;

        // h来正向表示每个点出去的连边.
        // value = 该连能连接到的点集合.
        let mut h: Vec<Vec<i32>> = Vec::new();
        h.resize(num_courses as usize, vec![]);
        // nodes记录所有的节点的入度.
        let mut nodes_in = Vec::new();
        nodes_in.resize(num_courses as usize, 0);
        // q用于广度优先遍历.
        let mut q = VecDeque::new();

        for edges in { prerequisites } {
            h[edges[1] as usize].push(edges[0]);
            nodes_in[edges[0] as usize] += 1;
        }

        // O(n)时间找到没有入度的起始节点, 入队列.
        for (point_id, in_cnt) in nodes_in.iter().enumerate() {
            if *in_cnt == 0 {
                q.push_back(point_id as i32);
            }
        }

        let mut ret = Vec::new();
        // 广度优先遍历.
        while !q.is_empty() {
            let point_id = q.pop_front().unwrap();
            ret.push(point_id);
            let nodes_to: &Vec<i32> = h.get(point_id as usize).unwrap();
            for node in nodes_to {
                let v: &mut i32 = nodes_in.get_mut(*node as usize).unwrap();
                *v -= 1;
                if *v == 0 {
                    q.push_back(*node);
                }
            }
        }
        if num_courses as usize == ret.len() {
            ret
        } else {
            vec![]
        }
    }
}