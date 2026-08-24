struct Solution;

impl Solution {
    pub fn stone_game_viii(stones: Vec<i32>) -> i32 {
        let n = stones.len();
        let mut prefix: Vec<i64> = vec![0; n + 1];
        for i in 0..n {
            prefix[i + 1] = prefix[i] + stones[i] as i64;
        }

        let mut dp: Vec<i64> = vec![0; n];
        dp[n - 1] = prefix[n];
        for i in (1..n - 1).rev() {
            dp[i] = (prefix[i + 1] - dp[i + 1]).max(dp[i + 1]);
        }

        dp[1] as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // 官方三個例子
    #[test]
    fn example_1() {
        // s = [-1, 1, -2, 2, -3]，Alice 搶 s[3]=2，Bob 被迫吃 s[4]=-3
        assert_eq!(Solution::stone_game_viii(vec![-1, 2, -3, 4, -5]), 5);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::stone_game_viii(vec![7, -6, 5, 10, 5, -2, -6]), 13);
    }

    #[test]
    fn example_3() {
        // n = 2，最小長度：Alice 被迫全拿，Bob 沒得動
        assert_eq!(Solution::stone_game_viii(vec![-10, -12]), -22);
    }

    // 全正數：前綴遞增，最大的一定在最後，所以一路跳到底再收
    #[test]
    fn all_positive_takes_everything() {
        assert_eq!(Solution::stone_game_viii(vec![1, 2, 3]), 6);
    }

    // 反向情況：尾巴是毒，Alice 必須馬上收手把 s[n-1] 丟給 Bob
    // 如果「跳過」那一項寫成 -dp[i+1]，這題會錯
    #[test]
    fn stops_early_when_tail_is_poison() {
        // s = [10, 11, -89]：拿 j=1 得 11，Bob 被迫吃 -89
        assert_eq!(Solution::stone_game_viii(vec![10, 1, -100]), 100);
    }

    // 迴圈必須停在 i = 1，回傳 dp[1]；回傳 dp[0] 會得到 10（第一手只拿一顆，非法）
    #[test]
    fn first_move_takes_at_least_two() {
        // s = [10, -90, -90]，dp[1] = 0 但 dp[0] = 10
        assert_eq!(Solution::stone_game_viii(vec![10, -100, 0]), 0);
    }

    // 前綴和到 1e9 量級，同時確認不是 O(n^2)
    #[test]
    fn large_input() {
        assert_eq!(
            Solution::stone_game_viii(vec![10000; 100_000]),
            1_000_000_000
        );
    }

    // 暴力法對照：小陣列直接展開所有遞增選法
    // best(last) = 上一手選了 last，換我選，我能拿到的最佳差值
    fn brute(s: &[i64], last: usize) -> i64 {
        let n = s.len();
        if last == n - 1 {
            return 0; // 只剩一顆，遊戲結束
        }
        (last + 1..n).map(|j| s[j] - brute(s, j)).max().unwrap()
    }

    fn xorshift(seed: &mut u64) -> u64 {
        *seed ^= *seed << 13;
        *seed ^= *seed >> 7;
        *seed ^= *seed << 17;
        *seed
    }

    #[test]
    fn matches_brute_force() {
        let mut seed = 88172645463325252u64;
        for _ in 0..300 {
            let n = 2 + (xorshift(&mut seed) % 7) as usize; // n = 2..=8
            let stones: Vec<i32> = (0..n)
                .map(|_| (xorshift(&mut seed) % 21) as i32 - 10) // -10..=10
                .collect();

            let mut s = Vec::with_capacity(n);
            let mut acc = 0i64;
            for &v in &stones {
                acc += v as i64;
                s.push(acc);
            }

            assert_eq!(
                Solution::stone_game_viii(stones.clone()),
                brute(&s, 0) as i32,
                "stones = {:?}",
                stones
            );
        }
    }
}
