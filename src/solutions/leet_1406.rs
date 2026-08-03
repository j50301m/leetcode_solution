struct Solution;

impl Solution {
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let n = stone_value.len();
        // dp[i] = 剩下 stone_value[i..] 時，當前玩家能拿到的最大分差（自己 - 對手）
        // dp[n] = 0 當哨兵，省掉邊界判斷
        let mut dp = vec![0i32; n + 1];

        for i in (0..n).rev() {
            let mut sum = 0;
            let mut best = i32::MIN;
            for k in 1..=3 {
                if i + k > n {
                    break;
                }
                sum += stone_value[i + k - 1]; // 累加拿走的 k 顆
                best = best.max(sum - dp[i + k]); // 換對手當「當前玩家」，故減號
            }
            dp[i] = best;
        }

        if dp[0] > 0 {
            "Alice"
        } else if dp[0] < 0 {
            "Bob"
        } else {
            "Tie"
        }
        .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::stone_game_iii(vec![1, 2, 3, -9]), "Alice");
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::stone_game_iii(vec![1, 2, 3, 7]), "Bob");
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::stone_game_iii(vec![1, 2, 3, 6]), "Tie");
    }
}
