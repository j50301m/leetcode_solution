struct Solution {}

impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        if n <= 2 {
            return n;
        }

        let len = n as usize;
        const MOD: i64 = 1_000_000_007;
        let mut f = vec![0i64; len + 1];
        let mut p = vec![0i64; len + 1];

        f[0] = 1;
        f[1] = 1;

        // 轉移：f[i] = f[i-1] + f[i-2] + 2·P[i-1]，
        // P[i] = P[i-1] + f[i-2]，i 從 2 開始算。
        for i in 2..=len {
            f[i] = (f[i - 1] + f[i - 2] + 2 * p[i - 1]) % MOD;
            p[i] = (p[i - 1] + f[i - 2] % MOD) % MOD;
        }

        f[len] as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::num_tilings(3), 5);
    }

    #[test]
    fn n_is_1() {
        assert_eq!(Solution::num_tilings(1), 1);
    }

    #[test]
    fn n_is_2() {
        assert_eq!(Solution::num_tilings(2), 2);
    }

    #[test]
    fn n_is_4() {
        assert_eq!(Solution::num_tilings(4), 11);
    }

    #[test]
    fn n_is_5() {
        assert_eq!(Solution::num_tilings(5), 24);
    }

    #[test]
    fn needs_modulo() {
        assert_eq!(Solution::num_tilings(30), 312342182);
        assert_eq!(Solution::num_tilings(1000), 979232805);
    }
}
