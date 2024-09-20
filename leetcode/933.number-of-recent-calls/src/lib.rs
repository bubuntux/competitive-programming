use std::collections::VecDeque;

struct RecentCounter {
    queue: VecDeque<i32>,
}

impl RecentCounter {
    fn new() -> Self {
        RecentCounter {
            queue: VecDeque::new(),
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        while !self.queue.is_empty() && self.queue[0] < t - 3000 {
            self.queue.pop_front();
        }
        self.queue.push_back(t);
        self.queue.len() as i32
    }
}

// https://leetcode.com/problems/number-of-recent-calls/

#[cfg(test)]
mod tests {
    use super::RecentCounter;

    #[test]
    fn example_1() {
        let mut rc = RecentCounter::new();
        assert_eq!(rc.ping(1), 1);
        assert_eq!(rc.ping(100), 2);
        assert_eq!(rc.ping(3001), 3);
        assert_eq!(rc.ping(3002), 3);
    }
}
