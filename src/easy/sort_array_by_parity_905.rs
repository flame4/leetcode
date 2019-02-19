use super::Solution;

impl Solution {
    pub fn sort_array_by_parity(a: Vec<i32>) -> Vec<i32> {
        // TODO: 使用迭代器和闭包来优化执行顺序.
        let mut b: Vec<i32> = Vec::new();
        b.resize(a.len(), 0);
        let mut start = 0;
        let mut end = a.len() - 1;
        for i in 0..a.len() {
            if a[i] % 2 == 0 {
                b[start] = a[i];
                start += 1;
            } else {
                b[end] = a[i];
                if end == 0 {
                    break;
                }
                end -= 1;
            }
        }
        b
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn sort_array_by_parity_1() {
        let a = vec![1];
        let b = Solution::sort_array_by_parity(vec![1]);
        assert_eq!(a.len(), b.len());
        for i in 0..a.len() {
            assert_eq!(a[i], b[i]);
        }
    }

    #[test]
    fn sort_array_by_parity_2() {
        let a = vec![2,4,1,3];
        let b = Solution::sort_array_by_parity(vec![3,1,2,4]);
        assert_eq!(a.len(), b.len());
        for i in 0..a.len() {
            assert_eq!(a[i], b[i]);
        }
    }
}
