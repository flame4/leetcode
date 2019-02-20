use super::Solution;

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut U_cnt = 0;
        let mut D_cnt = 0;
        let mut R_cnt = 0;
        let mut L_cnt = 0;

        for x in moves.chars() {
            match x {
                'U' => U_cnt += 1,
                'D' => D_cnt += 1,
                'R' => R_cnt += 1,
                'L' => L_cnt += 1,
                _ => (),
            }
        }

        (U_cnt == D_cnt) && (R_cnt == L_cnt)
    }
}