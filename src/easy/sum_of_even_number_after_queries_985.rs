use super::Solution;

impl Solution {

    // v1: 练习迭代器版本, 每次改动重新求和, 肯定很慢...
    pub fn sum_even_after_queries(a: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut a = a.clone();
        let mut ret = Vec::new();
        queries.iter().for_each(|x| {
            a[x[1] as usize] += x[0];
            ret.push(a.iter().filter(|x| {
                (*x) % 2 == 0
            }).sum())
        });
        ret
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn sum_even_after_queries_1() {
        assert_eq!(Solution::sum_even_after_queries(
            vec![1,2,3,4],
            vec![vec![1,0], vec![-3, 1], vec![-4, 0], vec![2,3]]),
        vec![8,6,2,4]);
    }
}