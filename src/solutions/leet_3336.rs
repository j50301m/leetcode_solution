struct Solution;

impl Solution {
    // 3336

    pub fn subsequence_pair_count(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;

        fn gcd(a: usize, b: usize) -> usize {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }

        // 因為constraint num = range[1,200]。 求GCD(seqA,seqb) 最多可能會有 201*201種組合
        let mut dp = vec![vec![0; 201]; 201];
        dp[0][0] = 1;

        for &x in nums.iter() {
            let x = x as usize;
            let mut new = vec![vec![0; 201]; 201];
            for a in 0..=200 {
                for b in 0..=200 {
                    let val = dp[a][b];
                    if val == 0 {
                        continue;
                    }

                    let case_1 = gcd(a, x);
                    let case_2 = gcd(b, x);
                    new[case_1][b] = (new[case_1][b] + val) % MOD;
                    new[a][case_2] = (new[a][case_2] + val) % MOD;
                    new[a][b] = (new[a][b] + val) % MOD;
                }
            }
            dp = new;
        }

        let mut ans = 0;
        for i in 1..=200 {
            ans = (ans + dp[i][i]) % MOD;
        }

        (ans % MOD) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn official_examples() {
        assert_eq!(Solution::subsequence_pair_count(vec![1, 2, 3, 4]), 10);
        assert_eq!(Solution::subsequence_pair_count(vec![10, 20, 30]), 2);
        assert_eq!(Solution::subsequence_pair_count(vec![1, 1, 1, 1]), 50);
    }

    #[test]
    fn edge_cases() {
        // 只有一個元素,湊不出兩個非空籃子
        assert_eq!(Solution::subsequence_pair_count(vec![3]), 0);
        // 手算過的例子:({3},{3}) 順序互換共 2 種
        assert_eq!(Solution::subsequence_pair_count(vec![3, 3]), 2);
        // 兩數 GCD 不同,無解
        assert_eq!(Solution::subsequence_pair_count(vec![4, 6]), 0);
    }

    #[test]
    fn big_case_needs_correct_mod() {
        // 200 個 1:答案 = (3^200 - 2*2^200 + 1) mod 1e9+7
        // 真實值遠超過 i32,MOD 常數錯或沒取模都會在這裡爆
        assert_eq!(Solution::subsequence_pair_count(vec![1; 200]), 137_428_029);
    }
}
