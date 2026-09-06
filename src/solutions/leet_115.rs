struct Solution {}

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let m = s.len();
        let n = t.len();
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut dp = vec![vec![0; n + 1]; m + 1]; // dp[m][n]: 由後往前第數
        for i in 0..m + 1 {
            dp[i][n] = 1;
        }

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                if s[i] == t[j] {
                    dp[i][j] = dp[i + 1][j] + dp[i + 1][j + 1];
                } else {
                    dp[i][j] = dp[i + 1][j];
                }
            }
        }

        dp[0][0]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn f(s: &str, t: &str) -> i32 {
        Solution::num_distinct(s.to_string(), t.to_string())
    }

    // 官方 example 1：rabbbit 裡有 3 種挑法拼出 rabbit
    #[test]
    fn case1() {
        assert_eq!(f("rabbbit", "rabbit"), 3);
    }

    // 官方 example 2：答案 5
    #[test]
    fn case2() {
        assert_eq!(f("babgbag", "bag"), 5);
    }

    // t 空字串 -> 1（什麼都不挑也是一種）
    // 抓 dp[i][n] = 1 這排初始化漏掉、或只設了 dp[m][n]
    #[test]
    fn case3() {
        assert_eq!(f("abc", ""), 1);
        assert_eq!(f("", ""), 1);
    }

    // s 空、t 不空 -> 0
    // 抓 m = 0 時 dp 只有一列、迴圈不跑的邊界
    #[test]
    fn case4() {
        assert_eq!(f("", "a"), 0);
    }

    // s 比 t 短 -> 0
    #[test]
    fn case5() {
        assert_eq!(f("ab", "abc"), 0);
    }

    // 完全相同 -> 只有 1 種
    #[test]
    fn case6() {
        assert_eq!(f("abc", "abc"), 1);
    }

    // t 根本不是 s 的子序列（順序反了）-> 0
    #[test]
    fn case7() {
        assert_eq!(f("abc", "cba"), 0);
    }

    // 全同字元：等於 C(4,2) = 6
    // 抓「字元相同時只加 dp[i+1][j+1]、忘了 dp[i+1][j] 這條不選 s[i] 的路」-> 會變 1
    #[test]
    fn case8() {
        assert_eq!(f("aaaa", "aa"), 6);
        assert_eq!(f("aaa", "a"), 3);
    }

    // 反過來：忘了字元相同時要加 dp[i+1][j+1]（真的配對那條）-> 會變 0
    #[test]
    fn case9() {
        assert_eq!(f("ab", "ab"), 1);
    }

    // 大小寫要當不同字元 -> 只有 1 種
    #[test]
    fn case10() {
        assert_eq!(f("aA", "a"), 1);
        assert_eq!(f("aA", "A"), 1);
    }

    // 中間夾雜無關字元，只有一種挑法
    #[test]
    fn case11() {
        assert_eq!(f("xaxbxcx", "abc"), 1);
    }

    // C(20,10) = 184756，用來壓 dp 遞推有沒有整段對
    #[test]
    fn case12() {
        assert_eq!(f(&"a".repeat(20), &"a".repeat(10)), 184_756);
    }
}
