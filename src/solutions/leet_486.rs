struct Solution;

impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut dp = vec![vec![0; n]; n];

        for i in (0..n).rev() {
            for j in i..n {
                if i == j {
                    dp[i][j] = nums[i];
                    continue;
                }

                let left = nums[i] - dp[i + 1][j];
                let right = nums[j] - dp[i][j - 1];
                dp[i][j] = left.max(right);
            }
        }

        dp[0][n - 1] >= 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        assert!(!Solution::predict_the_winner(vec![1, 5, 2]));
    }

    #[test]
    fn case_2() {
        assert!(Solution::predict_the_winner(vec![1, 5, 233, 7]));
    }

    #[test]
    fn case_3() {
        assert!(Solution::predict_the_winner(vec![3, 9, 1, 2]));
    }

    #[test]
    fn case_4() {
        assert!(Solution::predict_the_winner(vec![1]));
    }

    #[test]
    fn case_5() {
        assert!(Solution::predict_the_winner(vec![1, 5, 2, 4, 6]));
        assert!(Solution::predict_the_winner(vec![7, 7]));
    }

    #[test]
    fn case_6() {
        assert!(!Solution::predict_the_winner(vec![2, 4, 55, 6, 8]));
    }
}
