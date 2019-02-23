use super::Solution;

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut u_cnt = 0;
        let mut d_cnt = 0;
        let mut r_cnt = 0;
        let mut l_cnt = 0;

        for x in moves.chars() {
            match x {
                'U' => u_cnt += 1,
                'D' => d_cnt += 1,
                'R' => r_cnt += 1,
                'L' => l_cnt += 1,
                _ => (),
            }
        }

        (u_cnt == d_cnt) && (r_cnt == l_cnt)
    }
}