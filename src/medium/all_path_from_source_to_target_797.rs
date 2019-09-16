use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/all-paths-from-source-to-target/
    /// 给定一个 N 个节点的有向无环图, 要求找出所有的路径.
    /// 已经给定的便利条件: 起点是 0, 终点是 N - 1, 不需要再进行起点和终点判定.
    /// 思路为最朴素的深度遍历优先搜索, 为了记录曾经走过的节点, 加一个数组记录即可,
    /// 这个数组的初始值为 -1, 表示没有遇到走过这个节点, 否则记录寻址链路的下一个节点 id,
    /// 就是复用了这个数组来记录这条路径是如何走到结尾的.
    /// 为了记录所有可能的路径, 我们需要再用一个额外数组记录走过的路径.
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut recorder = vec![];
        recorder.resize(graph.len(), -1);
        let mut result = vec![];
        Solution::dfs_search(&graph, 0, &mut recorder, &mut result);
        result
    }

    fn dfs_search(graph: &Vec<Vec<i32>>, now: i32, recorder: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        // 确保 graph 和 recorder 初始化正确.
        assert_eq!(graph.len(), recorder.len());

        // with end.
        if now as usize == (graph.len() - 1) {
            result.push(Solution::get_path_from_recorder(recorder))
        }

        // dfs search
        for next_node in graph.get(now as usize).unwrap() {
            // 已经访问过的节点 pass 掉.
            if recorder[*next_node as usize] != -1 {
                continue
            }
            recorder[now as usize] = *next_node;
            Solution::dfs_search(graph, *next_node, recorder, result);
            // 别忘了恢复 recorder 空路径.
            recorder[now as usize] = -1;
        }
    }

    #[allow(unreachable_code)]
    fn get_path_from_recorder(recorder: &Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        let mut node = 0;
        loop {
            result.push(node);
            if node as usize == (recorder.len() - 1) {
                return result;
            }
            node = recorder[node as usize];
        }
        unreachable!()
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    pub fn all_path_source_target_test() {
        assert_eq!(Solution::all_paths_source_target(vec![vec![1,2], vec![3], vec![3], vec![]]),
                   vec![vec![0, 1, 3], vec![0, 2, 3]]);

        assert_eq!(Solution::all_paths_source_target(
            vec![
                vec![2, 4],
                vec![5],
                vec![1, 3],
                vec![1, 4, 5],
                vec![5],
                vec![]
            ]
        ),
        vec![
            vec![0, 2, 1, 5],
            vec![0, 2, 3, 1, 5],
            vec![0, 2, 3, 4, 5],
            vec![0, 2, 3, 5],
            vec![0, 4, 5]
        ])
    }

}

