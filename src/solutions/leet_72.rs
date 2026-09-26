struct Solution {}

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let b1 = word1.as_bytes();
        let b2 = word2.as_bytes();
        let m = b1.len();
        let n = b2.len();
        let mut dp = vec![vec![0i32; n + 1]; m + 1];
        for i in 1..=m {
            dp[i][0] = i as i32;
        }
        for j in 1..=n {
            dp[0][j] = j as i32;
        }

        for i in 1..=m {
            for j in 1..=n {
                let cost = if b1[i - 1] == b2[j - 1] { 0 } else { 1 };

                dp[i][j] = (dp[i - 1][j - 1] + cost)
                    .min(dp[i - 1][j] + 1)
                    .min(dp[i][j - 1] + 1);
            }
        }

        dp[m][n] as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn run(a: &str, b: &str) -> i32 {
        Solution::min_distance(a.to_string(), b.to_string())
    }

    #[test]
    fn example1() {
        assert_eq!(run("horse", "ros"), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(run("intention", "execution"), 5);
    }

    // 對稱：a→b 和 b→a 距離相同
    #[test]
    fn symmetric() {
        assert_eq!(run("ros", "horse"), 3);
        assert_eq!(run("execution", "intention"), 5);
    }

    // 空字串：只能全插入 / 全刪除（測 dp 第 0 行、第 0 列的初始化）
    #[test]
    fn empty() {
        assert_eq!(run("", ""), 0);
        assert_eq!(run("", "abc"), 3);
        assert_eq!(run("abc", ""), 3);
    }

    #[test]
    fn identical() {
        assert_eq!(run("abc", "abc"), 0);
    }

    // 完全不同、等長：全部替換
    #[test]
    fn all_replace() {
        assert_eq!(run("abc", "xyz"), 3);
    }

    // 各自只靠一種操作
    #[test]
    fn single_op() {
        assert_eq!(run("abc", "abxc"), 1); // 插入
        assert_eq!(run("abxc", "abc"), 1); // 刪除
        assert_eq!(run("abc", "axc"), 1); // 替換
    }

    // 順序顛倒：不能「交換」，要 2 步
    #[test]
    fn reversed() {
        assert_eq!(run("ab", "ba"), 2);
    }
}
