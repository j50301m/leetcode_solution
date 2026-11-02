struct Solution {}

impl Solution {
    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        nums.sort();

        let mid = if n % 2 == 0 {
            ((nums[n / 2] + nums[n / 2 - 1]) / 2) as i64
        } else {
            nums[n / 2] as i64
        };

        let mut count = 0;
        for &num in nums.iter() {
            count += (mid - num as i64).abs();
        }

        count as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::min_moves2(vec![1, 0, 0, 8, 6]), 14);
    }
}
