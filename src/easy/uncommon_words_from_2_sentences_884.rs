use crate::Solution;
use std::collections::HashSet;

impl Solution {
    // TODO 内存使用比较多.
    pub fn uncommon_from_sentences(a: String, b: String) -> Vec<String> {
        let (a_words, a_keys) = Solution::get_uncommon(a);
        let (b_words, b_keys) = Solution::get_uncommon(b);
        let unique_set = a_words.symmetric_difference(&b_words)
            .into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>();
        let common_part = a_keys.intersection(&b_keys)
            .into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>();
        unique_set.difference(&common_part).into_iter().map(|s| s.to_owned()).collect::<Vec<String>>()
    }

    /// 第一个是s内只出现一次的单词, 第二个是s内出现的所有单词.
    fn get_uncommon(s: String) -> (HashSet<String>, HashSet<String>) {
        use std::collections::HashMap;
        let words = s.split_whitespace().map(|s| s.to_owned()).collect::<Vec<String>>();
        let mut m = HashMap::new();
        for word in words {
            if let Some(times) = m.get_mut(&word) {
                *times += 1;
            } else {
                m.insert(word, 1);
            }
        }

        let p0 = m.iter().filter(|item| {
            *(*item).1 == 1
        }).map(|s| s.0.to_owned()).collect::<HashSet<String>>();
        let p1 = m.keys().into_iter().map(|s| s.to_owned()).collect::<HashSet<String>>();
        (p0, p1)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    pub fn uncommon_from_sentences_test() {
        assert_eq!(Solution::uncommon_from_sentences("s z z z s".to_string(), "s z ejt".to_string()),
        vec!["ejt".to_string()]);
    }
}


