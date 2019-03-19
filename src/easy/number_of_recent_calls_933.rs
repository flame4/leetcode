use std::collections::vec_deque::VecDeque;

struct RecentCounter {
    pings: VecDeque<i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl RecentCounter {

    fn new() -> Self {
        let mut ret = RecentCounter {
            pings: VecDeque::new()
        };
        ret.pings.reserve(10000);
        ret
    }

    // TODO speed up.
    pub fn ping(&mut self, t: i32) -> i32 {
        self.pings.push_back(t);
        while let Some(x) = self.pings.front() {
            if t - x > 3000 {
                self.pings.pop_front();
            } else {
                break
            }
        }
        self.pings.len() as i32
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */


#[cfg(test)]
mod tests {
    use super::RecentCounter;
    #[test]
    pub fn number_of_recent_calls_1() {
        let mut obj = RecentCounter::new();
        assert_eq!(obj.ping(100), 1);
        assert_eq!(obj.ping(3500), 1);
        assert_eq!(obj.ping(3800), 2);
    }
}