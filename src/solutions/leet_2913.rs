use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        for i in 0..nums.len() {
            let mut set = HashSet::new();
            for j in i..nums.len() {
                set.insert(nums[j]);
                let l = set.len() as i32;
                sum += l * l;
            }
        }
        sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::sum_counts(vec![1, 2, 1]), 15);
    }
}
