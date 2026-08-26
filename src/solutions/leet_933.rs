struct RecentCounter {
    queue: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {
    fn new() -> Self {
        Self { queue: Vec::new() }
    }

    fn ping(&mut self, t: i32) -> i32 {
        let min = t - 3000;
        self.queue.push(t);
        while let Some(&x) = self.queue.first() {
            if x < min {
                self.queue.remove(0);
            } else {
                break;
            }
        }
        self.queue.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let mut c = RecentCounter::new();
        assert_eq!(c.ping(1), 1);
        assert_eq!(c.ping(100), 2);
        assert_eq!(c.ping(3001), 3);
        assert_eq!(c.ping(3002), 3);
    }

    #[test]
    fn boundary_kept() {
        // t - 3000 剛好等於舊值，屬於 [t-3000, t] 區間，要留下
        let mut c = RecentCounter::new();
        assert_eq!(c.ping(1), 1);
        assert_eq!(c.ping(3001), 2);
    }

    #[test]
    fn boundary_dropped() {
        // 差 3001，超出區間，要丟掉
        let mut c = RecentCounter::new();
        assert_eq!(c.ping(1), 1);
        assert_eq!(c.ping(3002), 1);
    }

    #[test]
    fn all_expired() {
        let mut c = RecentCounter::new();
        c.ping(1);
        c.ping(2);
        c.ping(3);
        assert_eq!(c.ping(100000), 1);
    }
}
