use crate::Solution;

impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let n = s.len();
        let mut min = 0;
        let mut max = n as i32;
        let mut ret = Vec::new();
        ret.resize(n+1, 0);
        for (i, x) in s.chars().enumerate() {
            if x == 'I' {
                ret[i] = min;
                min += 1;
            } else {
                ret[i] = max;
                max -= 1;
            }
        }
        ret[n] = min;
        ret
    }
}