struct Solution {}

impl Solution {
    pub fn max_sum(nums: Vec<i32>, k: i32) -> i32 {
        const MOD: i64 = 1000000007;
        let mut bit_cnt = vec![0i32; 31];
        let mut sum = 0;
        for num in nums.iter() {
            let mut num = *num;
            while num > 0 {
                let x = num & (!num + 1);
                let idx = x.ilog2() as usize;
                bit_cnt[idx] += 1;
                num -= x;
            }
        }

        for _ in 0..k {
            let mut x = 0;
            for (idx, cnt) in bit_cnt.iter_mut().enumerate() {
                if *cnt == 0 {
                    continue;
                }
                *cnt -= 1;
                x += 2i64.pow(idx as u32);
            }
            x = x % MOD;
            sum = (sum + x * x) % MOD;
        }

        (sum % MOD) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        // LeetCode 範例 1：15² + 6²
        assert_eq!(Solution::max_sum(vec![2, 6, 5, 8], 2), 261);
    }

    #[test]
    fn case2() {
        // LeetCode 範例 2：不需要任何操作，直接挑 7, 5, 4
        assert_eq!(Solution::max_sum(vec![4, 5, 4, 7], 3), 90);
    }

    #[test]
    fn case3() {
        // 所有 bit 都能堆到同一個數：1111 = 15，第二個數是 0
        assert_eq!(Solution::max_sum(vec![2, 5, 8], 2), 225);
    }

    #[test]
    fn case4() {
        // 同一欄有 3 個 1，k = n，每列各拿一個
        assert_eq!(Solution::max_sum(vec![1, 1, 1], 3), 3);
    }

    #[test]
    fn case5() {
        // 組出 2^30 - 1，超過 MOD，需要先取 mod 再平方
        assert_eq!(Solution::max_sum(vec![536870912, 536870911], 1), 388912871);
    }

    #[test]
    fn case6() {
        // 1e5 個 1e9，每個平方約 1e18，累加時沒有每次取 mod 會溢位
        assert_eq!(
            Solution::max_sum(vec![1_000_000_000; 100_000], 100_000),
            4900000
        );
    }
}
