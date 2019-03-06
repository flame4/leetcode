use crate::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn repeated_n_times(a: Vec<i32>) -> i32 {
        let mut m = HashSet::new();
        for i in a {
            if !m.contains(&i) {
                m.insert(i);
                continue
            }
            return i
        }
        0
    }
}


#[cfg(test)]
mod tests {

    use crate::Solution;

    #[test]
    pub fn repeated_n_times_1() {
        assert_eq!(5, Solution::repeated_n_times(vec![5,1,5,2,5,3,5,4]))
    }
}