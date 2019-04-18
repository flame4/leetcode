use crate::Solution;

impl Solution {
    /// https://leetcode.com/problems/word-search/
    /// 在一个二维的数字阵列里面找到是不是有那么一个单词, 但是这个单词的构成不是必须横的或者竖立的, 是可以拐弯的.
    /// 这题目就是典型的简单的DFS搜索即可, 为了复用空间, 我们不copy一个数组, 而是把原数组的值改为一个标记值.
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut board = { board };
        let chars = { word }.chars().into_iter().collect::<Vec<char>>();
        for y in 0..board.len() {
            for x in 0..board[0].len() {
                if Solution::exist_dfs(x, y, &chars, 0, &mut board) {
                    return true;
                }
            }
        }
        false
    }

    /// x: 当前访问的x
    /// y: 当前访问的y
    /// chars 要找的chars
    /// char_index 当前要找的是chars的哪个char.
    /// board 迷宫图.
    fn exist_dfs(x: usize, y: usize, chars: &Vec<char>, char_index: usize, board: &mut Vec<Vec<char>>) -> bool {
        if board[y][x] != chars[char_index] {
            return false;
        }
        let tmp = board[y][x];
        board[y][x] = '#';
        // 找到了一个单词!
        if char_index + 1 == chars.len() {
            return true;
        }
        // up
        if y > 0 && board[y - 1][x] != '#' && Solution::exist_dfs(x, y - 1, chars, char_index + 1, board) {
            return true;
        }
        // down
        if y + 1 < board.len() && board[y + 1][x] != '#' && Solution::exist_dfs(x, y + 1, chars, char_index + 1, board) {
            return true;
        }
        // left
        if x > 0 && board[y][x - 1] != '#' && Solution::exist_dfs(x - 1, y, chars, char_index + 1, board) {
            return true;
        }
        // right
        if x + 1 < board[y].len() && board[y][x + 1] != '#' && Solution::exist_dfs(x + 1, y, chars, char_index + 1, board) {
            return true;
        }

        // 记得把设为"#"的改回去.
        board[y][x] = tmp;
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    pub fn exist_test() {
        let v = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E']
        ];
        assert_eq!(Solution::exist(v.clone(), "ABCCED".to_string()), true);
        assert_eq!(Solution::exist(v.clone(), "SEE".to_string()), true);
        assert_eq!(Solution::exist(v.clone(), "ABCB".to_string()), false);
    }
}

