use std::collections::BTreeSet;

pub struct Solution;

impl Solution {
    pub fn contains_nearby_almost_duplicate(
        nums: Vec<i32>,
        index_diff: i32,
        value_diff: i32,
    ) -> bool {
        let mut window: BTreeSet<i32> = BTreeSet::new();

        for i in 0..nums.len() {
            let num = nums[i];
            if window
                .range((num - value_diff)..=(num + value_diff))
                .next()
                .is_some()
            {
                return true;
            }

            // Insert current num
            window.insert(num);

            if i >= index_diff as usize {
                window.remove(&nums[i - index_diff as usize]);
            }
        }

        return false;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let result = Solution::contains_nearby_almost_duplicate(vec![1, 2, 3, 1], 3, 0);
        assert!(result);
    }

    #[test]
    fn case_2() {
        let result = Solution::contains_nearby_almost_duplicate(vec![1, 5, 9, 1, 5, 9], 2, 3);
        assert!(!result);
    }
}
