use crate::Solution;
use std::collections::HashMap;
use std::collections::vec_deque::VecDeque;

impl Solution {
    /// https://leetcode.com/problems/word-ladder/
    /// 给定两个string, 以及一列string, 求一条最短的转换链路, 从begin转换到end.
    /// 一次转换定义为更换一个字母.
    /// 首先,这个问题要用广度优先遍历来做,最简单的方法就是维护两个队列. 每次从已遍历集合里面拿出一个来，
    /// 并且去另一个里面查找. 时间复杂度为 O(M * N^2), M是字符串长度, N是数组的长度.
    /// 这里面的点在于对于每一步的BFS, 我们都需要遍历的从未访问集合里面找到对应的元素集合.
    /// 那么优化的思路也很简单, 我们想办法用Map来做快速查找方式, 让搜索的时间变为O(1)即可.
    /// 具体的办法是: 对于每个字母扩展一个可能的集合key.
    /// 例如, "abcd" => "*bcd", "a*cd", "ab*d", "abc*"
    /// 然后用这些对象做key, value是word list, 来查找相互之间的关联. 那么这一步需要的额外空间就是 O(M * N).
    /// 空间复杂度的计算方法: 每个单词有M个要放的地方, 一共有N个单词.
    /// 要注意的一点是, end word是不能加入到word_list内的.
    /// faster than 88.24%.
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        // 标记哪个是end_word,这样判断停止的时候不需要再比较了.
        let w = word_list.iter().map(|s| s.as_str()).collect();
        // 记录每个能通过变换达到的路径.
        let all_comb_list = Solution::pre_deal(w, &end_word);
        // 引用的默认比较方式是比较引用值, 如果要比较本身, 要转换为 as_ptr()
        let mut visited = word_list.iter().map(|s| (s.as_str(), false)).collect::<HashMap<&str, bool>>();
        let mut q = VecDeque::new();
        q.push_back((begin_word.as_str(), 1));
        while let Some(item) = q.pop_front() {
            let mut change = item.0.to_owned();
            for i in 0..change.len() {
                let tmp;
                unsafe {
                    tmp = change.as_bytes()[i];
                    change.as_bytes_mut()[i] = '*' as u8;
                }
                if let Some(word) = all_comb_list.get(&change) {
                    // 把所有对象入队列, 记得查找已经访问过的就别放进去了, 直到找到一个end_word.
                    for i in word {
                        if end_word == *i.0 {
                            return item.1 + 1;
                        }
                        let t = visited.get_mut(i.0).unwrap();
                        if !*t {
                            *t = true;
                            q.push_back((i.0, item.1 + 1));
                        }
                    }
                }
                unsafe {
                    change.as_bytes_mut()[i] = tmp;
                }
            }
        }
        0
    }


    /// 预处理过程
    /// 这里注意看一下Hashmap内因为存的是只读数据, 所以可以用引用来存放, 防止空间的copy浪费.
    fn pre_deal<'a>(word_list: Vec<&'a str>, end_word: &'a str) -> HashMap<String, Vec<(&'a str, bool)>> {
        let mut ret: HashMap<String, Vec<(&'a str, bool)>> = HashMap::new();

        // 把中间元素入队列.
        for i in word_list {
            let mut change = i.to_owned();
            let is_end = change == *end_word;
            for j in 0..change.len() {
                let tmp;
                unsafe {
                    tmp = change.as_bytes()[j];
                    change.as_bytes_mut()[j] = '*' as u8;
                }
                match ret.contains_key(&change) {
                    true => {
                        ret.get_mut(&change).unwrap().push((i, is_end));
                    }
                    false => {
                        ret.insert(change.clone(), vec![(i, is_end)]);
                    }
                }
                unsafe {
                    change.as_bytes_mut()[j] = tmp;
                }
            }
        }
        ret
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn find_median_sorted_arrays_test() {
        assert_eq!(Solution::ladder_length("hit".to_string(), "cog".to_string(),
                                           vec!["hot".to_string(), "dot".to_string(), "dog".to_string(), "lot".to_string(), "log".to_string(), "cog".to_string()]), 5);
        assert_eq!(Solution::ladder_length("hit".to_string(), "cog".to_string(),
                                           vec!["hot".to_string(), "dot".to_string(), "dog".to_string(), "lot".to_string(), "log".to_string()]), 0);
    }
}
