use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/unique-paths-iii/
    /// 思路很简单, 可以递归做, 申请一个实时记录当前哪些位置被走过.
    /// 因为给定矩阵长度为20, 所以我们可以判断最多这个栈只有400层.不会爆炸.
    /// 我们可以复用原来的数组空间, 就不需要重新申请位置了.
    /// 1 = start, 2 = end, 0 = way, -1 = obstacle
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        let mut cnt = 0;
        let y_size = grid.len();
        if grid.is_empty() || grid.get(0).unwrap().is_empty() {
            return cnt;
        }
        let x_size = grid.get(0).unwrap().len();
        let mut start_x= 0;
        let mut start_y= 0;
        // 走遍所有的格子一共要走多少格子, 也就是grid内0, 1的个数之和.
        let mut need_visit = 0;
        let mut has_visit = 0;
        for y in 0..y_size {
            for x in 0..x_size {
                if grid.get(y).unwrap()[x] == 0 {
                    need_visit += 1;
                }
                if grid.get(y).unwrap()[x] == 1 {
                    start_x = x;
                    start_y = y;
                    need_visit += 1;
                }
            }
        }
        Solution::recursive_unique_path_iii(start_x, start_y, &mut {grid}, &mut cnt, &mut has_visit, need_visit);
        cnt
    }

    fn recursive_unique_path_iii(x: usize, y: usize, grid: &mut Vec<Vec<i32>>,
                                 cnt: &mut i32, has_visit: &mut i32, need_visit: i32) {
        match grid.get(y).unwrap()[x] {
            0 | 1 => {
                *(grid.get_mut(y).unwrap().get_mut(x).unwrap()) = -1;
                *has_visit += 1;
                // TODO 可以优化一次寻址读取.
                if x > 0 && (grid.get(y).unwrap()[x-1] == 0 || grid.get(y).unwrap()[x-1] == 2) {
                    // LEFT
                    Solution::recursive_unique_path_iii(x-1, y, grid, cnt, has_visit, need_visit);
                }
                if x < grid.get(0).unwrap().len() - 1
                    && (grid.get(y).unwrap()[x+1] == 0 || grid.get(y).unwrap()[x+1] == 2){
                    // RIGHT
                    Solution::recursive_unique_path_iii(x+1, y, grid, cnt, has_visit, need_visit);
                }
                if y > 0 && (grid.get(y-1).unwrap()[x] == 0 || grid.get(y-1).unwrap()[x] == 2){
                    // UP
                    Solution::recursive_unique_path_iii(x, y-1, grid, cnt, has_visit, need_visit);
                }
                if y < grid.len() - 1 && (grid.get(y+1).unwrap()[x] == 0 || grid.get(y+1).unwrap()[x] == 2) {
                    // DOWN
                    Solution::recursive_unique_path_iii(x, y+1, grid, cnt, has_visit, need_visit);
                }
                *(grid.get_mut(y).unwrap().get_mut(x).unwrap()) = 0;
                *has_visit -= 1;
            },
            2 => {
                if *has_visit == need_visit {
                    *cnt += 1;
                }
            },
            _ => unreachable!()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn unique_path_iii_test() {
        let grid = vec![
            vec![1,0,0,0],
            vec![0,0,0,0],
            vec![0,0,2,-1],
        ];
        assert_eq!(Solution::unique_paths_iii(grid), 2);

        let grid = vec![
            vec![1,0,0,0],
            vec![0,0,0,0],
            vec![0,0,0,2],
        ];
        assert_eq!(Solution::unique_paths_iii(grid), 4);

        let grid = vec![
            vec![0,1],
            vec![2,0],
        ];
        assert_eq!(Solution::unique_paths_iii(grid), 0);
    }
}
