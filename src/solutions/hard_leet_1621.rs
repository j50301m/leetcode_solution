struct Solution {}

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn number_of_sets(n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        // dp[j] = 用前 i 個點畫 j 條線段，且所有右端點 <= 第 i-1 個點
        // s[j]  = dp[1][j] + ... + dp[i][j]，前綴和取代原本的內層迴圈
        let mut dp = vec![0i64; k + 1];
        let mut s = vec![0i64; k + 1];
        dp[0] = 1;

        for _ in 1..=n {
            // j 由大到小跑，讀 s[j-1] 時它還是上一輪的值
            for j in (1..=k).rev() {
                dp[j] = (dp[j] + s[j - 1]) % MOD;
                s[j] = (s[j] + dp[j]) % MOD;
            }
            s[0] += 1; // dp[0] 恆為 1
        }

        dp[k] as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // 基本例：4 個點畫 2 條，5 種當中有 4 種是共用端點的
    #[test]
    fn case1() {
        assert_eq!(Solution::number_of_sets(4, 2), 5);
    }

    // 只畫 1 條 = 從 3 個點裡挑 2 個當端點
    #[test]
    fn case2() {
        assert_eq!(Solution::number_of_sets(3, 1), 3);
    }

    // 會超過 i32，逼出取模
    #[test]
    fn case3_needs_mod() {
        assert_eq!(Solution::number_of_sets(30, 7), 796297179);
    }

    // k 塞到最滿：每條線段都只能長 1，只有一種畫法
    #[test]
    fn case4_packed() {
        assert_eq!(Solution::number_of_sets(5, 4), 1);
    }

    // 最小輸入
    #[test]
    fn case5_min() {
        assert_eq!(Solution::number_of_sets(2, 1), 1);
    }

    // n 上限，確認不會爆或退化
    #[test]
    fn case6_large_n() {
        assert_eq!(Solution::number_of_sets(1000, 1), 499500);
    }
}
