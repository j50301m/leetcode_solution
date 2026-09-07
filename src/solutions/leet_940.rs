struct Solution {}

impl Solution {
    pub fn distinct_subseq_ii(s: String) -> i32 {
        const MOD: i64 = 10i64.pow(9) + 7;
        let n = s.len();
        let s = s.as_bytes();
        let mut dp = vec![0i64; n + 1];
        let mut last_pos: Vec<i32> = vec![-1; 26];

        dp[0] = 1;

        for i in 0..n {
            let pos = (s[i] - b'a') as usize;
            if last_pos[pos] == -1 {
                dp[i + 1] = (2 * dp[i]) % MOD
            } else {
                dp[i + 1] = (2 * dp[i] - dp[last_pos[pos] as usize]) % MOD
            }
            last_pos[pos] = i as i32;
        }

        let ans = dp[n] - 1;
        if ans < 0 {
            return (ans + MOD) as i32;
        }

        ans as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::distinct_subseq_ii("abc".to_string()), 7);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::distinct_subseq_ii("aba".to_string()), 6);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::distinct_subseq_ii("aaa".to_string()), 3);
    }
}
