struct Solution;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let mut sum = 0;
        for i in 0..k {
            sum += nums[i];
        }

        let mut avg = sum as f64 / k as f64;

        for i in k..nums.len() {
            sum = sum + nums[i] - nums[i - k];
            avg = avg.max(sum as f64 / k as f64);
        }

        avg
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 4),
            12.75
        );
    }

    #[test]
    fn case_2() {
        let result = Solution::find_max_average(vec![0, 4, 0, 3, 2], 1);
        assert_eq!(result, 4f64);
    }
}
