struct Solution;

impl Solution {
    pub fn stone_game_v(stone_value: Vec<i32>) -> i32 {
        let n = stone_value.len();
        let mut dp = vec![vec![0; n]; n];

        let mut prefix = vec![0; n + 1];
        for i in 0..n {
            prefix[i + 1] = prefix[i] + stone_value[i];
        }

        for i in (0..n).rev() {
            for j in i..n {
                if i == j {
                    dp[i][j] = 0;
                    continue;
                }

                let mut best = 0;
                for k in 0..(j - i) {
                    // left = [i..=i+k];
                    // right =[i+k+1..=j];
                    let split = i + k + 1;
                    let left = prefix[split] - prefix[i];
                    let right = prefix[j + 1] - prefix[split];

                    if left < right {
                        best = best.max(left + dp[i][i + k]);
                    } else if right < left {
                        best = best.max(right + dp[i + k + 1][j]);
                    } else {
                        let dp = dp[i][i + k].max(dp[i + k + 1][j]);
                        best = best.max(left + dp);
                    }
                }

                dp[i][j] = best;
            }
        }

        dp[0][n - 1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::stone_game_v(vec![6, 2, 3, 4, 5, 5]), 18);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::stone_game_v(vec![7, 7, 7, 7, 7, 7, 7]), 28);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::stone_game_v(vec![4]), 0);
    }

    #[test]
    fn case_4() {
        assert_eq!(Solution::stone_game_v(vec![1, 2]), 1);
        assert_eq!(Solution::stone_game_v(vec![3, 4]), 3);
    }

    // 右半段的和必須是「補集」，不是左半段的鏡像長度
    #[test]
    fn case_5() {
        assert_eq!(Solution::stone_game_v(vec![6, 2, 3]), 7);
        assert_eq!(Solution::stone_game_v(vec![6, 2, 3, 4]), 10);
    }

    // 兩半相等時 Alice 自己選邊：留 [2,2] 得 4+2=6，留 [1,3] 只有 4+1=5
    #[test]
    fn case_6() {
        assert_eq!(Solution::stone_game_v(vec![2, 2, 1, 3]), 6);
    }
}
