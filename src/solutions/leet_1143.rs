struct Solution {}

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let m = text1.len();
        let n = text2.len();

        let text1 = text1.as_bytes();
        let text2 = text2.as_bytes();
        let mut dp = vec![vec![0; n + 1]; m + 1];

        for i in 1..=m {
            let t1 = text1[i - 1];
            for j in 1..=n {
                let t2 = text2[j - 1];

                if t1 == t2 {
                    dp[i][j] = 1 + dp[i - 1][j - 1];
                } else {
                    dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
                }
            }
        }

        dp[m][n]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::longest_common_subsequence("abcdef".to_string(), "ade".to_string()),
            3
        );
    }

    fn run(a: &str, b: &str) -> i32 {
        Solution::longest_common_subsequence(a.to_string(), b.to_string())
    }

    #[test]
    fn example1() {
        assert_eq!(run("abcde", "ace"), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(run("abc", "abc"), 3);
    }

    #[test]
    fn example3() {
        assert_eq!(run("abc", "def"), 0);
    }

    #[test]
    fn single_char() {
        assert_eq!(run("a", "a"), 1);
    }

    // 同一個字元不能被配對兩次
    #[test]
    fn repeated_char_one_side() {
        assert_eq!(run("aa", "a"), 1);
        assert_eq!(run("a", "aa"), 1);
    }

    // 順序不同：只能取其中一個
    #[test]
    fn reversed() {
        assert_eq!(run("ab", "ba"), 1);
    }

    #[test]
    fn different_lengths() {
        assert_eq!(run("bsbininm", "jmjkbkjkv"), 1);
    }
}
