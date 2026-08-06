use std::{cmp::Reverse, collections::BinaryHeap};

struct Solution;

impl Solution {
    pub fn min_build_time(blocks: Vec<i32>, split: i32) -> i32 {
        let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        blocks.into_iter().for_each(|i| min_heap.push(Reverse(i)));

        while min_heap.len() > 1 {
            let Reverse(first) = min_heap.pop().unwrap();
            let Reverse(second) = min_heap.pop().unwrap();
            min_heap.push(Reverse(first.max(second) + split));
        }

        let Reverse(ans) = min_heap.pop().unwrap();
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::min_build_time(vec![1, 2], 5), 7);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::min_build_time(vec![1, 2, 3], 1), 4);
    }

    #[test]
    fn single_block() {
        assert_eq!(Solution::min_build_time(vec![1], 1), 1);
    }

    #[test]
    fn expensive_split() {
        assert_eq!(Solution::min_build_time(vec![1, 2, 3], 5), 12);
    }

    #[test]
    fn balanced_tree() {
        assert_eq!(Solution::min_build_time(vec![4, 5, 6, 7], 2), 11);
    }
}
