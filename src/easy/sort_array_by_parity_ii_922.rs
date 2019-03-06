use crate::Solution;


impl Solution {
    // 一半奇数, 一半偶数的数组, 调整后返回的数组. 奇数index位置为奇数, 偶数index为偶数.
    // O(n)时间复杂度, 一次遍历数组, O(1)空间复杂度.
    // 维护两个index, 在奇数和偶数上跳转.
    pub fn sort_array_by_parity_ii(a: Vec<i32>) -> Vec<i32> {
        // TODO 这是个什么写法?
        // let mut a = &mut {a};
        let mut a = a.clone();
        let mut odd_index = 1;
        let mut even_index = 0;
        while odd_index < a.len() {
            if a[odd_index] % 2 == 0 {
                while a[even_index] % 2 == 0 {
                    even_index += 2;
                }
                a.swap(odd_index, even_index);
            }
            odd_index += 2;
        }
        a
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    pub fn sort_array_by_parity_ii_1() {
        let ret = Solution::sort_array_by_parity_ii(vec![4,2,5,7]);
        for (index, value) in ret.into_iter().enumerate() {
            assert_eq!((index as i32 + value) %2, 0)
        }

        let ret = Solution::sort_array_by_parity_ii(vec![2,1,4,3,6,5,8,7]);
        for (index, value) in ret.into_iter().enumerate() {
            assert_eq!((index as i32 + value) %2, 0)
        }

        let ret = Solution::sort_array_by_parity_ii(vec![3,4]);
        for (index, value) in ret.into_iter().enumerate() {
            assert_eq!((index as i32 + value) %2, 0)
        }
    }
}