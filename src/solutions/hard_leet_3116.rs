struct Solution;

impl Solution {
    pub fn find_kth_smallest(coins: Vec<i32>, k: i32) -> i64 {
        let coins: Vec<u64> = coins.iter().map(|&c| c as u64).collect();
        let k = k as u64;

        // 下界 1。
        // 上界：光用最小面額就能湊出 k 個金額（min, 2*min, ..., k*min），
        // 所以答案不可能大於 k * min。k 最大 2e9、min 最大 25 → 5e10，
        // 這裡一定要 u64/i64，i32 會直接溢位。
        let mut lo: u64 = 1;
        let mut hi: u64 = k * coins.iter().copied().min().unwrap();

        // 對答案二分：找「最小的 x 使 count_le(x) >= k」。
        while lo < hi {
            // 中點是 lo + 寬度的一半。寫成 (hi - lo) / 2 會得到「寬度的一半」，
            // 那個值可能落在 lo 左邊，lo = mid + 1 反而讓 lo 倒退 → 無窮迴圈。
            let mid = lo + (hi - lo) / 2;

            if Self::count_le(&coins, mid) < k {
                lo = mid + 1; // mid 太小，連 k 個都湊不到 → 答案在右邊，mid 可以排除
            } else {
                hi = mid; // mid 已達標，但它自己可能就是答案 → 保留，不要寫 mid - 1
            }
        }

        // 收斂時 count_le(lo) >= k 而 count_le(lo - 1) < k，兩者只差 1 個元素，
        // 那個元素就是 lo 本身 → lo 一定是可達金額，不必再修正。
        lo as i64
    }

    /// 「<= x 且至少被 coins 裡某個面額整除」的金額個數。
    ///
    /// 因為不能混用面額，可達集合就是各面額倍數的聯集。聯集大小用容斥原理算：
    ///   count(x) = Σ (S 為非空子集) (-1)^(|S|+1) * ⌊x / lcm(S)⌋
    /// 也就是「奇數個面額的交集加、偶數個減」。
    fn count_le(coins: &[u64], x: u64) -> u64 {
        let n = coins.len();

        // 中途會是負的（減掉重疊時），用 u64 會 underflow：
        // debug build 直接 panic、release 靜靜 wrap 成天文數字。
        let mut total: i64 = 0;

        // mask 從 1 開始，不是 0：空集合不在公式裡。
        // （空集合的 lcm 是 1，會把 1..=x 全部算進來，一開始就錯到底。）
        for mask in 1u32..(1u32 << n) {
            // mask 的第 i 個 bit 是 1 → 這個子集包含 coins[i]。
            // 邊掃邊滾 lcm，就得到「同時是子集內每個面額的倍數」的週期。
            let mut l: u64 = 1;
            for i in 0..n {
                if mask >> i & 1 == 1 {
                    l = lcm(l, coins[i]);
                }
            }

            // 同時是 a、b、... 的倍數 ⇔ 是 lcm(a, b, ...) 的倍數（不是乘積！），
            // 所以這個交集裡 <= x 的個數就是 ⌊x / l⌋。整數除法自動幫我們取整。
            let term = (x / l) as i64;

            // 子集大小的奇偶決定正負；位元數交給 count_ones,別自己數。
            if mask.count_ones() % 2 == 1 {
                total += term;
            } else {
                total -= term;
            }
        }

        // 聯集大小不可能是負的，收尾轉回 u64 是安全的。
        total as u64
    }
}

// 輾轉相除；先除再乘，避免 a*b 溢位
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1_redundant_coins() {
        // 6、9 都是 3 的倍數，一個新金額都沒帶進來 → 集合就是 3 的倍數，第 3 小是 9
        assert_eq!(Solution::find_kth_smallest(vec![3, 6, 9], 3), 9);
    }

    #[test]
    fn example_2_overlap_removed() {
        // 2,4,5,6,8,10,12 → 第 7 小是 12。10 同時是 2 和 5 的倍數，只能算一次。
        // 這組就是舊的 mid = (hi - lo) / 2 會卡在 lo = 5, hi = 14 無窮迴圈的 case。
        assert_eq!(Solution::find_kth_smallest(vec![5, 2], 7), 12);
    }

    #[test]
    fn count_le_matches_hand_trace() {
        // coins = [2,3,5], x = 20：10 + 6 - 3 + 4 - 2 - 1 + 0 = 14
        // （直接相加會是 20，多算 6 個 —— 這就是要容斥的原因）
        assert_eq!(Solution::count_le(&[2, 3, 5], 20), 14);
        assert_eq!(Solution::find_kth_smallest(vec![2, 3, 5], 14), 20);
    }

    #[test]
    fn upper_bound_needs_i64() {
        // hi = k * 1 = 2e9，已經超過 i32::MAX，上界用 i32 這裡就爆
        assert_eq!(Solution::find_kth_smallest(vec![1], 2_000_000_000), 2_000_000_000);
    }

    #[test]
    fn answer_exceeds_i32() {
        // 答案 5e10 本身就放不進 i32，回傳型別是 i64 有其道理
        assert_eq!(
            Solution::find_kth_smallest(vec![25], 2_000_000_000),
            50_000_000_000
        );
    }

    #[test]
    fn k_is_one() {
        // 邊界：lo 從 1 起跳，答案是最小面額 4
        assert_eq!(Solution::find_kth_smallest(vec![7, 4], 1), 4);
    }
}
