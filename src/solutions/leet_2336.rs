use std::collections::BTreeSet;

struct SmallestInfiniteSet {
    curr_number: i32,
    used_set: BTreeSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SmallestInfiniteSet {
    fn new() -> Self {
        Self {
            curr_number: 1,
            used_set: BTreeSet::new(),
        }
    }

    fn pop_smallest(&mut self) -> i32 {
        if let Some(num) = self.used_set.pop_first() {
            return num;
        }
        let pop = self.curr_number;
        self.curr_number += 1;
        pop
    }

    fn add_back(&mut self, num: i32) {
        if num < self.curr_number {
            self.used_set.insert(num);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let mut s = SmallestInfiniteSet::new();
        s.add_back(2);
        assert_eq!(s.pop_smallest(), 1);
        assert_eq!(s.pop_smallest(), 2);
        assert_eq!(s.pop_smallest(), 3);
        s.add_back(1);
        assert_eq!(s.pop_smallest(), 1);
        assert_eq!(s.pop_smallest(), 4);
        assert_eq!(s.pop_smallest(), 5);
    }

    #[test]
    fn pops_are_consecutive() {
        let mut s = SmallestInfiniteSet::new();
        for expected in 1..=5 {
            assert_eq!(s.pop_smallest(), expected);
        }
    }

    #[test]
    fn add_back_number_still_present_is_a_no_op() {
        let mut s = SmallestInfiniteSet::new();
        s.add_back(5);
        for expected in 1..=6 {
            assert_eq!(s.pop_smallest(), expected);
        }
    }

    #[test]
    fn add_back_twice_only_returns_it_once() {
        let mut s = SmallestInfiniteSet::new();
        assert_eq!(s.pop_smallest(), 1);
        assert_eq!(s.pop_smallest(), 2);
        s.add_back(1);
        s.add_back(1);
        assert_eq!(s.pop_smallest(), 1);
        assert_eq!(s.pop_smallest(), 3);
    }

    #[test]
    fn added_back_numbers_come_out_in_order() {
        let mut s = SmallestInfiniteSet::new();
        for _ in 0..5 {
            s.pop_smallest();
        }
        s.add_back(4);
        s.add_back(2);
        assert_eq!(s.pop_smallest(), 2);
        assert_eq!(s.pop_smallest(), 4);
        assert_eq!(s.pop_smallest(), 6);
    }
}
