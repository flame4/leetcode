use crate::Solution;

impl Solution {
    pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut b = Vec::new();
        for i in 0..a.len() {
            let tmp = &a[i];
            let mut cell = Vec::new();
            cell.resize(tmp.len(), 0);
            for j in 0..tmp.len() {
                cell[j] = 1 - tmp[tmp.len()- 1 - j];
            }
            b.push(cell);
        }
        b
    }
}