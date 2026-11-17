struct Solution {}

impl Solution {
    pub fn minimum_changes(s: String, k: i32) -> i32 {
        const IMPOSSIBLE: i32 = i32::MAX / 2;
        let n = s.len();
        let k = k as usize;
        let bytes = s.as_bytes();

        let mut segment_cost = vec![vec![IMPOSSIBLE; n]; n]; // [start][end]
        for start in 0..n {
            for end in start + 1..n {
                segment_cost[start][end] = Self::min_cost(&bytes[start..=end]);
            }
        }

        let mut dp = vec![vec![IMPOSSIBLE; n + 1]; k + 1];
        dp[0][n] = 0;
        for i in 1..k + 1 {
            for start in 0..n {
                for end in start + 1..n {
                    let rest = dp[i - 1][end + 1];
                    if rest == IMPOSSIBLE {
                        continue;
                    }
                    dp[i][start] = dp[i][start].min(segment_cost[start][end] + rest);
                }
            }
        }

        dp[k][0]
    }

    fn min_cost(s: &[u8]) -> i32 {
        let n = s.len();
        let mut min_cost = n as i32;
        for divisor in 1..=n / 2 {
            if n % divisor != 0 {
                continue;
            }
            min_cost = min_cost.min(Self::cost(s, divisor));
        }

        min_cost
    }

    fn cost(s: &[u8], divisor: usize) -> i32 {
        let mut cost = 0;
        for i in 0..divisor {
            let mut left = i;
            let mut right = left + s.len() - divisor;

            while left < right {
                if s[left] != s[right] {
                    cost += 1;
                }
                left += divisor;
                right -= divisor;
            }
        }

        cost
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_changes() {
        assert_eq!(Solution::minimum_changes("abcac".to_string(), 2), 1);
        assert_eq!(Solution::minimum_changes("abcdef".to_string(), 2), 2);
        assert_eq!(Solution::minimum_changes("aabbaa".to_string(), 3), 0);
        assert_eq!(Solution::minimum_changes("abbaab".to_string(), 1), 0);
        assert_eq!(Solution::minimum_changes("ab".to_string(), 1), 1);
    }

    #[test]
    fn test_cost() {
        // "abbaab" 對照前面的表格
        assert_eq!(Solution::cost(b"abbaab", 1), 3);
        assert_eq!(Solution::cost(b"abbaab", 2), 0);
        assert_eq!(Solution::cost(b"abbaab", 3), 1);
    }

    #[test]
    fn test_min_cost() {
        assert_eq!(Solution::min_cost(b"abbaab"), 0);
        assert_eq!(Solution::min_cost(b"abab"), 0);
        assert_eq!(Solution::min_cost(b"ab"), 1);
    }
}
