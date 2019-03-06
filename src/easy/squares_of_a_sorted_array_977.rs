use crate::Solution;

impl Solution {
    pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
        let mut tmp = Vec::new();
        let mut ret = Vec::new();
        for i in 0..a.len() {
            tmp.push(a[i] * a[i])
        }

        let mut begin = 0;
        let mut end = tmp.len()-1;

        while begin <= end {
            if tmp[begin] > tmp[end] {
                ret.push(tmp[begin]);
                begin += 1;
            } else {
                ret.push(tmp[end]);
                if end == 0 {
                    break
                }
                end -= 1;
            }
        }

        ret.reverse();
        ret
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn sorted_squares_1() {
        // TODO: how to judge Same Vec?
        let a = vec![0, 1, 9, 16, 100];
        let b =  Solution::sorted_squares(vec![-4, -1, 0, 3, 10]);
        assert_eq!(a.len(), b.len());
        for i in 0..a.len() {
            assert_eq!(a[i], b[i]);
        }
    }

    #[test]
    pub fn sorted_squares_2() {
        // TODO: how to judge Same Vec?
        let a = vec![1];
        let b =  Solution::sorted_squares(vec![1]);
        assert_eq!(a.len(), b.len());
        for i in 0..a.len() {
            assert_eq!(a[i], b[i]);
        }
    }
}