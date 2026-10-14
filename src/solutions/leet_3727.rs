struct Solution {}

impl Solution {
    pub fn max_alternating_sum(mut nums: Vec<i32>) -> i64 {
        // nums.sort_by(|a, b| a.abs().cmp(&b.abs()));

        let mid = nums.len() / 2;
        nums.select_nth_unstable_by_key(mid, |value| value.abs());
        let positive_sum = nums[mid..]
            .iter()
            .fold(0i64, |acc, &x| acc + (x as i64 * x as i64));

        let negative_sum = nums[0..mid]
            .iter()
            .fold(0i64, |acc, &x| acc + (x as i64 * x as i64));

        positive_sum - negative_sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::max_alternating_sum(vec![2, 1, 3]), 12);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::max_alternating_sum(vec![1, -1, 2, -2, 3, -3]), 16);
    }
}
