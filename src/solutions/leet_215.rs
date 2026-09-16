use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::from(nums);

        let mut curr_pop = None;
        for _ in 0..k {
            curr_pop = heap.pop();
        }

        curr_pop.unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
    }
}
