struct Solution;

impl Solution {
    pub fn winner_square_game(n: i32) -> bool {
        let n = n as usize;
        let mut dp = vec![false; n + 1];
        for i in 1..=n {
            let mut k = 1;
            while k * k <= i {
                if !dp[i - k * k] {
                    dp[i] = true;
                    break;
                }
                k += 1;
            }
        }

        return dp[n];
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // 手算驗證過的必勝/必敗局面
    #[test]
    fn cases() {
        let cases: &[(i32, bool)] = &[
            (1, true),   // 拿 1，對手面對 0
            (2, false),  // 只能拿 1，對手拿走最後一顆
            (3, true),   // 拿 1 剩 2（必敗態）丟給對手
            (4, true),   // 平方數，一次拿光
            (5, false),  // 拿 1 剩 4、拿 4 剩 1，都是對手必勝
            (6, true),   // 拿 1 剩 5
            (7, false),  // LeetCode 範例 3
            (8, true),   // 拿 1 剩 7；貪心拿最大的 4 反而會輸
            (9, true),   // 平方數
            (10, false), // 三種走法全部把必勝態送給對手
            (17, false),
            (25, true), // 平方數，打破前面 0/2 mod 5 的必敗規律
        ];
        for (n, want) in cases {
            assert_eq!(Solution::winner_square_game(*n), *want, "n = {}", n);
        }
    }

    // 直接照定義寫的純遞迴（沒有 dp 陣列、沒有計算順序的假設），當真值來源
    // n = 0 時 iterator 為空，any 回傳 false，剛好就是「面對 0 顆必敗」
    fn brute(n: usize) -> bool {
        (1..)
            .map(|k| k * k)
            .take_while(|&s| s <= n)
            .any(|s| !brute(n - s))
    }

    #[test]
    fn small_matches_brute_force() {
        for n in 1..=35 {
            assert_eq!(Solution::winner_square_game(n as i32), brute(n), "n = {}", n);
        }
    }

    // 題目上界，確認不會 panic / index 越界
    #[test]
    fn upper_bound_does_not_panic() {
        Solution::winner_square_game(100_000);
    }
}
