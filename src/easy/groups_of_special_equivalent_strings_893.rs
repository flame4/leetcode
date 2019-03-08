use crate::Solution;

impl Solution {
    /// 判断两个字符是不是special equivalent是关键.
    /// 关键就是记录其组成部分是不是相等.
    /// 题目限定26个字符, 所以奇数偶数位一共有长度为52的就足够表征一个string的特征了.
    pub fn num_special_equiv_groups(a: Vec<String>) -> i32 {
        use std::collections::HashSet;
        let mut counter = HashSet::new();
        for i in a {
            counter.insert(Solution::get_feature(i));
        }
        counter.len() as i32
    }


    fn get_feature(s: String) -> Vec<i32> {
        // How to create a array with no default value.
        // https://www.joshmcguigan.com/blog/array-initialization-rust/
        let mut ret = Vec::new();
        ret.resize(52, 0);
        for (index, val) in s.as_bytes().into_iter().enumerate() {
            let index = (*val - 'a' as u8) as usize + 26 * (index % 2) as usize;
            ret[index] += 1;
        }
        ret
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    pub fn num_special_equiv_groups_test() {
        // TODO 从leetcode上抄下来再手动加to_string有点麻烦.
        assert_eq!(3, Solution::num_special_equiv_groups(
                       vec![
                           "ed".to_string(),
                           "lo".to_string(),
                           "eo".to_string(),
                           "lo".to_string(),
                           "lo".to_string()
                       ]
                   ));
    }
}