struct Solution {}

impl Solution {
    pub fn maximum_books(books: Vec<i32>) -> i64 {
        let n = books.len();
        let mut dp = vec![0i64; n];
        // 棧裡放 index，key = books[i] - i 由底到頂嚴格遞增
        let mut stack: Vec<usize> = Vec::new();
        let key = |i: usize| books[i] as i64 - i as i64;

        let mut ans = 0;
        for i in 0..n {
            // key 比我大或一樣的都彈掉：它們離右端點更遠又不比我高，之後誰來都先撞到 i
            while stack.last().is_some_and(|&top| key(top) >= key(i)) {
                stack.pop();
            }

            let b = books[i] as i64;
            let val = match stack.last() {
                // 有 j：(j, i] 這 d 格是等差，左邊直接接上算好的 dp[j]
                Some(&j) => dp[j] + Self::arith(b, (i - j) as i64),
                // 棧空：一路等差到底，撞到 index 0 或拿到 0 本，看誰先到
                None => Self::arith(b, (i as i64 + 1).min(b)),
            };

            dp[i] = val;
            stack.push(i);
            ans = ans.max(val);
        }
        ans
    }

    /// first + (first-1) + ... + (first-cnt+1)
    /// = 長方形 cnt*first 減掉階梯 0+1+...+(cnt-1)
    fn arith(first: i64, cnt: i64) -> i64 {
        cnt * first - cnt * (cnt - 1) / 2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // O(n^2) 對照組：每個 r 都從右往左老實掃一遍，慢但一定對
    fn brute(books: &[i32]) -> i64 {
        let mut best = 0;
        for r in 0..books.len() {
            let mut take = books[r] as i64;
            let mut total = take;
            for i in (0..r).rev() {
                take = (books[i] as i64).min(take - 1);
                if take <= 0 {
                    break;
                }
                total += take;
            }
            best = best.max(total);
        }
        best
    }

    // 官方 example 1：拿 1 + 2 + 7 + 9
    #[test]
    fn case1() {
        assert_eq!(Solution::maximum_books(vec![8, 5, 2, 7, 9]), 19);
    }

    // 官方 example 2：中間有個 0，區間被切斷，只能拿右邊三格 3 + 4 + 5
    #[test]
    fn case2() {
        assert_eq!(Solution::maximum_books(vec![7, 0, 3, 4, 5]), 12);
    }

    // 官方 example 3：最佳解在中間（r = 3），不是最右邊 -> 抓「只看最後一格」
    #[test]
    fn case3() {
        assert_eq!(Solution::maximum_books(vec![8, 2, 3, 7, 3, 4, 0, 1, 4, 3]), 13);
    }

    // 只有一格
    #[test]
    fn case4() {
        assert_eq!(Solution::maximum_books(vec![1]), 1);
    }

    // 全空書架 -> 0。抓「ans 初始值拿 dp[0] 以外的東西」或忘了 0 這個下限
    #[test]
    fn case5() {
        assert_eq!(Solution::maximum_books(vec![0, 0, 0]), 0);
    }

    // 全是 1：嚴格遞增讓你只能挑一格 -> 1
    #[test]
    fn case6() {
        assert_eq!(Solution::maximum_books(vec![1, 1, 1, 1]), 1);
    }

    // 等差和公式的主場：9+8+7+6+5 = 35（i = 4 時 cnt = 5）
    // 抓「cnt 只寫 books[i]、忘了會先撞到 index 0」：那樣 i = 0 會算成 9*9 - 36 = 45
    #[test]
    fn case7() {
        assert_eq!(Solution::maximum_books(vec![9, 9, 9, 9, 9]), 35);
    }

    // 本來就嚴格遞增 -> 整段全拿
    #[test]
    fn case8() {
        assert_eq!(Solution::maximum_books(vec![1, 2, 3, 4, 5]), 15);
    }

    // 棧空時 cnt 若只寫 i + 1（忘了會先拿到 0 本），這裡會算成 1 + 0 + (-1) = 0
    #[test]
    fn case9() {
        assert_eq!(Solution::maximum_books(vec![0, 0, 1]), 1);
    }

    // 遞減陣列：往左最多只撐得住兩格，5 -> 4+3 = 7
    #[test]
    fn case10() {
        assert_eq!(Solution::maximum_books(vec![5, 4, 3, 2, 1]), 7);
    }

    // 10^5 個 10^5：答案 5,000,050,000 遠超過 i32 -> 抓中途用 i32 乘法
    #[test]
    fn case11() {
        let n = 100_000i64;
        let ans = Solution::maximum_books(vec![100_000; n as usize]);
        assert_eq!(ans, n * 100_000 - n * (n - 1) / 2);
        assert!(ans > i32::MAX as i64);
    }

    // 跟暴力版對拍 2000 組隨機小陣列
    #[test]
    fn case12() {
        let mut seed = 0x2355u64;
        let mut rnd = |m: u64| {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            ((seed >> 33) % m) as i32
        };
        for _ in 0..2000 {
            let len = rnd(9) + 1;
            let v: Vec<i32> = (0..len).map(|_| rnd(8)).collect();
            assert_eq!(Solution::maximum_books(v.clone()), brute(&v), "{v:?}");
        }
    }
}
