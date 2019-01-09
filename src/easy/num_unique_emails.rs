use super::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut set: HashSet<String> = HashSet::new();
        for i in 0..emails.len() {
            let mut email = emails[i].chars();
            let mut real_email = String::new();
            let mut meet_at: bool = false;
            let mut meet_plus: bool = false;
            // TODO 简化代码.

            let mut byte = email.next();
            while byte != None {
                let b = byte.unwrap();
                byte = email.next();

                if meet_at {
                    real_email.push(b);
                    continue
                }

                if b == '@' {
                    meet_at = true;
                    continue
                }

                if b == '+' {
                    meet_plus = true;
                    continue
                }

                if meet_plus  {
                    continue
                }

                if b == '.' {
                    continue
                }
                real_email.push( b);
            }
            set.insert(real_email);
        }
        set.len() as i32
    }
}




#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn num_unique_emails_1() {
        let mut data = Vec::new();
        // TODO &str vs String.
        data.push(String::from("test.email+alex@leetcode.com"));
        data.push(String::from("test.e.mail+bob.cathy@leetcode.com"));
        data.push(String::from("testemail+david+@lee.tcode.com"));
        assert_eq!(2, Solution::num_unique_emails(data));
    }
}