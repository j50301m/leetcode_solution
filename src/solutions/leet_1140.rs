struct Solution;

impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let n = piles.len();

        // suffix[i] = total stones ,when idx at i.
        let mut suffix = vec![0; n + 1];
        for i in (0..n).rev() {
            suffix[i] = suffix[i + 1] + piles[i];
        }

        // BUild a dp[i][j] i:remaining stone index, j = M start from 0 (0 means M=1)
        let mut dp = vec![vec![-1; n]; n];
        for i in (0..n).rev() {
            for j in (0..n).rev() {
                if 2 * (j + 1) >= n - i {
                    dp[i][j] = suffix[i];
                    continue;
                }

                let mut best = 0;
                for k in 1..=2 * (j + 1) {
                    let m = (j + 1).max(k) - 1;
                    let tmp = suffix[i] - dp[i + k][m];
                    best = best.max(tmp);
                }
                dp[i][j] = best;
            }
        }

        dp[0][0]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // 手算驗證過的期望值
    #[test]
    fn cases() {
        let cases: &[(&[i32], i32)] = &[
            (&[2, 7, 9, 4, 4], 10),       // LeetCode 範例 1：先讓一步才是最佳
            (&[1, 2, 3, 4, 5, 100], 104), // LeetCode 範例 2：忍到最後吃 100
            (&[7], 7),                    // n=1，一次拿光
            (&[1, 2], 3),                 // n=2，2M=2 剛好蓋滿
            (&[1, 2, 3], 3),              // n=3，第一次不是 base case
            (&[1, 2, 3, 4], 5),           // n=4
            (&[1; 8], 4),                 // 全等值，剛好對半
        ];
        for (piles, want) in cases {
            assert_eq!(
                Solution::stone_game_ii(piles.to_vec()),
                *want,
                "piles = {:?}",
                piles
            );
        }
    }

    // 沒記憶化的純遞迴，當作真值來源
    fn brute(piles: &[i32], i: usize, m: usize) -> i32 {
        let n = piles.len();
        if i >= n {
            return 0;
        }
        let suffix: i32 = piles[i..].iter().sum();
        (1..=2 * m)
            .take_while(|&x| i + x <= n)
            .map(|x| suffix - brute(piles, i + x, m.max(x)))
            .max()
            .unwrap()
    }

    // 窮舉 n=1..6、值 1..3 的所有陣列，跟暴力解對答案
    #[test]
    fn exhaustive_small() {
        for n in 1..=6usize {
            for code in 0..3usize.pow(n as u32) {
                let mut c = code;
                let piles: Vec<i32> = (0..n)
                    .map(|_| {
                        let v = (c % 3) as i32 + 1;
                        c /= 3;
                        v
                    })
                    .collect();
                assert_eq!(
                    Solution::stone_game_ii(piles.clone()),
                    brute(&piles, 0, 1),
                    "piles = {:?}",
                    piles
                );
            }
        }
    }
}
