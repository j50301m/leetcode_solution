struct Solution {}

impl Solution {
    pub fn result_array(nums: Vec<i32>, k: i32) -> Vec<i64> {
        let n = nums.len();
        let nums: Vec<i64> = nums.into_iter().map(|x| x as i64).collect();
        let k = k as i64;
        let mut dp = vec![0i64; k as usize];
        let mut results = vec![0i64; k as usize];
        let mut ndp = vec![0i64; k as usize];
        for i in 0..n {
            let remainder = nums[i] % k;
            ndp[remainder as usize] = 1;

            // 算跟別人的乘積
            for j in 0..k {
                let r = (nums[i] * j as i64) % k;

                let r_cnt = dp[j as usize];
                ndp[r as usize] += r_cnt;
            }

            for j in 0..k as usize {
                results[j] += ndp[j];
                (dp[j], ndp[j]) = (ndp[j], 0i64);
            }
        }

        results
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::result_array(vec![1, 2, 3, 4, 5], 3),
            vec![9, 2, 4]
        );
    }
}
