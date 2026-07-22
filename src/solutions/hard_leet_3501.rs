struct Solution;

impl Solution {
    pub fn max_active_sections_after_trade(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = s.len();
        let bytes = s.as_bytes();
        // 答案基底:整條 s 的 1 總數(交易只是被限制在子字串內做,計分算全字串)
        let total_ones = bytes.iter().filter(|&&b| b == b'1').count();

        // ---------- 預處理(整個函式只做一次) ----------

        // 1. 收集所有 0 區塊。它們天然有序,且任兩個相鄰 0 區塊中間必夾一個 1 區塊
        let mut zero_blocks: Vec<(usize, usize)> = Vec::new();
        let mut i = 0;
        while i < n {
            let start = i;
            while i < n && bytes[i] == bytes[start] {
                i += 1;
            }
            if bytes[start] == b'0' {
                zero_blocks.push((start, i - 1));
            }
        }
        let z = zero_blocks.len();

        // 2. pair_gain[k] = 第 k、k+1 個 0 區塊的「完整長度」和
        //    = 在這一對上交易(移除中間 1 區塊、合併後全填 1)的淨增加
        let pair_gain: Vec<usize> = (1..z)
            .map(|k| {
                let (s0, e0) = zero_blocks[k - 1];
                let (s1, e1) = zero_blocks[k];
                (e0 - s0 + 1) + (e1 - s1 + 1)
            })
            .collect();

        // 3. 對 pair_gain 建 sparse table(倍增法 RMQ):
        //    st[j][k] = pair_gain[k .. k + 2^j] 的最大值
        //    建表 O(z log z),之後查任意區間最大值 O(1)。
        //    為什麼可以查「完整長度」?因為 query 裁切只會影響最左、最右兩個
        //    區塊,中間的區塊一定完整落在區間內(見下方查詢處的說明)。
        let m = pair_gain.len();
        let mut st: Vec<Vec<usize>> = vec![pair_gain];
        let mut len = 2;
        while len <= m {
            let prev = st.last().unwrap();
            let row: Vec<usize> = (0..=m - len)
                .map(|k| prev[k].max(prev[k + len / 2]))
                .collect();
            st.push(row);
            len <<= 1;
        }
        // 閉區間 [lo, hi] 的最大值:用兩段長 2^j 的重疊區間蓋住整段
        let range_max = |lo: usize, hi: usize| -> usize {
            let j = (hi - lo + 1).ilog2() as usize;
            st[j][lo].max(st[j][hi + 1 - (1 << j)])
        };

        // ---------- 每個 query O(log n) ----------

        let mut ans = Vec::with_capacity(queries.len());
        for query in queries {
            let (l, r) = (query[0] as usize, query[1] as usize);

            // 二分找出「與 [l, r] 有交集」的 0 區塊索引範圍 [a, b]:
            // 區塊的 start、end 都遞增,所以兩個條件都可以 partition_point。
            // a  = 第一個 end >= l 的區塊
            // b_pp = start <= r 的區塊數量(即 b = b_pp - 1)
            let a: usize = zero_blocks.partition_point(|&(_, e)| e < l);
            let b_pp = zero_blocks.partition_point(|&(s0, _)| s0 <= r);

            // 交集區塊不到兩個 → 湊不出一對,無法交易
            if b_pp < a + 2 {
                ans.push(total_ones as i32);
                continue;
            }
            let b = b_pp - 1;

            // 關鍵觀察:只有區塊 a(可能左邊超出 l)和區塊 b(可能右邊超出 r)
            // 會被裁切;a+1 ..= b-1 的區塊必定完整在 [l, r] 內。
            // 證:區塊 a+1 的 start > 區塊 a 的 end >= l;區塊 b-1 的 end < 區塊 b 的 start <= r。
            let clip = |k: usize| -> usize {
                let (s0, e0) = zero_blocks[k];
                e0.min(r) - s0.max(l) + 1
            };

            // 含邊界區塊的兩對,用裁切後長度手算(b = a+1 時兩者是同一對)
            let mut best = clip(a) + clip(a + 1);
            best = best.max(clip(b - 1) + clip(b));
            // 完全不含邊界區塊的 pair(索引 a+1 ..= b-2)直接查 sparse table
            if b >= a + 3 {
                best = best.max(range_max(a + 1, b - 2));
            }

            ans.push((total_ones + best) as i32);
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let s = String::from("01");
        let quires = vec![vec![0, 1]];
        let result = Solution::max_active_sections_after_trade(s, quires);
        assert_eq!(result, vec![1])
    }

    #[test]
    fn case_2() {
        let s = String::from("0100");
        let quires = vec![vec![0, 3], vec![0, 2], vec![1, 3], vec![2, 3]];
        let result = Solution::max_active_sections_after_trade(s, quires);
        assert_eq!(result, vec![4, 3, 1, 1])
    }

    #[test]
    fn case_3() {
        let s = String::from("1000100");
        let quires = vec![vec![1, 5], vec![0, 6], vec![0, 4], vec![2, 3]];
        let result = Solution::max_active_sections_after_trade(s, quires);
        assert_eq!(result, vec![6, 7, 2, 2])
    }

    // Bug 1 迴歸測試:全字串有 2 個 1,區間內無法交易 → 答案是 2 不是 1
    #[test]
    fn ones_outside_range() {
        let s = String::from("0110");
        let result = Solution::max_active_sections_after_trade(s, vec![vec![2, 3]]);
        assert_eq!(result, vec![2])
    }

    // Bug 2 迴歸測試:區間外的區塊被誤納入時會 usize 下溢 panic
    #[test]
    fn block_after_range() {
        let s = String::from("0101");
        let result = Solution::max_active_sections_after_trade(s, vec![vec![0, 1]]);
        assert_eq!(result, vec![2])
    }

    // 中間 pair 要走 sparse table 的情況(區間內有 4 個 0 區塊)
    #[test]
    fn interior_pair_wins() {
        // 0 區塊長度依序為 1, 3, 3, 1 → 最佳是中間那對 3+3=6,total_ones=3
        let s = String::from("01000100010");
        let result = Solution::max_active_sections_after_trade(s, vec![vec![0, 10]]);
        assert_eq!(result, vec![9])
    }
}
