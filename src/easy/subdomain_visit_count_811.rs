use crate::Solution;
use std::collections::HashMap;

impl Solution {
    // 练习字符串的基本操作.
    // 其实在用的时候, 会发现对String, &str用起来迷迷糊糊的, 不知道哪里是引用, 哪里是
    // 所有权转移, 而哪里又发生了真正的数据拷贝.
    // TODO 在最后通过的代码里, 添加注释详细分析上面提到的问题.
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        let mut m = HashMap::new();
        for s in cpdomains {
            let mut ret = s.split_whitespace();
            let cnt: usize = ret.next().unwrap().parse::<usize>().unwrap();
            let url = ret.next().unwrap().to_owned();
            let mut now = String::new();
            for domain in url.rsplit(".") {
                if now.len() > 0 {
                    now.insert(0, '.');
                }
                now = domain.to_owned() + &now;
                if let Some(v) = m.get_mut(&now) {
                    *v += cnt;
                } else {
                    m.insert(now.clone(), cnt);
                }
            }
        }
        m.into_iter().map(|(url, cnt)| {
            format!("{}", cnt) + " " + &url
        }).collect::<Vec<String>>()
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;
    use std::collections::HashMap;

    #[test]
    pub fn subdomain_visits_1() {
        let s = vec![
            String::from("900 google.mail.com"),
            String::from("50 yahoo.com"),
            String::from("1 intel.mail.com"),
            String::from("5 wiki.org")
        ];
        let mut result: HashMap<&'static str, usize> = HashMap::new();
        result.insert("mail.com", 901);
        result.insert("yahoo.com", 50);
        result.insert("google.mail.com", 900);
        result.insert("wiki.org", 5);
        result.insert("org", 5);
        result.insert("intel.mail.com", 1);
        result.insert("com", 951);
        let ret = Solution::subdomain_visits(s);
        assert_eq!(ret.len(), 7);

        ret.into_iter().for_each(|x| {
            let mut iter = x.split_whitespace();
            let num: usize = iter.next().unwrap().parse().unwrap();
            let content = iter.next().unwrap();
            assert_eq!(result.get(content), Some(&num));
        });
    }
}