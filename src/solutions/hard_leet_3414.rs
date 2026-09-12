struct Solution {}

impl Solution {
    pub fn maximum_weight(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let n = intervals.len();

        let mut indices: Vec<(i32, i32, i64, i32)> = intervals // (left,right,weight,index)
            .into_iter()
            .enumerate()
            .map(|(idx, val)| (val[0], val[1], val[2] as i64, idx as i32))
            .collect();
        indices.sort_by_key(|&(l, r, _w, idx)| (r, l, idx));

        let ends: Vec<i32> = indices.iter().map(|&(_l, r, _w, _idx)| r).collect();

        // i64: max sum, Vec<i32>, construct indies
        let mut dp: Vec<Vec<(i64, Vec<i32>)>> = vec![vec![(0, Vec::new()); 5]; n + 1];
        for i in 1..n + 1 {
            let (l, _r, w, idx) = indices[i - 1];
            let p = ends.partition_point(|&x| x < l);
            for j in 1..5 {
                let (pre_sum, pre_indies) = dp[i - 1][j].clone();

                let curr_sum = dp[p][j - 1].0 + w;
                let pos = dp[p][j - 1].1.partition_point(|&x| x < idx);
                let mut curr_indies = dp[p][j - 1].1.clone();

                curr_indies.insert(pos, idx);
                if curr_sum > pre_sum || (curr_sum == pre_sum && curr_indies < pre_indies) {
                    dp[i][j] = (curr_sum, curr_indies);
                } else {
                    dp[i][j] = (pre_sum, pre_indies)
                }
            }
        }

        std::mem::take(&mut dp[n][4].1)
        // dp[n][4].1.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let intervals = vec![
            vec![1, 3, 2],
            vec![4, 5, 2],
            vec![1, 5, 5],
            vec![6, 9, 3],
            vec![6, 7, 1],
            vec![8, 9, 1],
        ];
        assert_eq!(Solution::maximum_weight(intervals), vec![2, 3]);
    }

    #[test]
    fn case_2() {
        let intervals = vec![
            vec![5, 8, 1],
            vec![6, 7, 7],
            vec![4, 7, 3],
            vec![9, 10, 6],
            vec![7, 8, 2],
            vec![11, 14, 3],
            vec![3, 5, 5],
        ];
        assert_eq!(Solution::maximum_weight(intervals), vec![1, 3, 5, 6]);
    }

    #[test]
    fn case_single() {
        assert_eq!(Solution::maximum_weight(vec![vec![1, 1, 9]]), vec![0]);
    }

    #[test]
    fn case_all_overlap_picks_heaviest() {
        let intervals = vec![vec![1, 10, 3], vec![2, 9, 5], vec![3, 8, 5]];
        // 5 與 5 同分，取字典序小的 index
        assert_eq!(Solution::maximum_weight(intervals), vec![1]);
    }

    #[test]
    fn case_more_than_four_available() {
        // 5 段互不重疊，只能挑 4 段，應丟掉最輕的 index 2
        let intervals = vec![
            vec![1, 1, 5],
            vec![2, 2, 5],
            vec![3, 3, 1],
            vec![4, 4, 5],
            vec![5, 5, 5],
        ];
        assert_eq!(Solution::maximum_weight(intervals), vec![0, 1, 3, 4]);
    }

    // 暴力解：列舉所有 <=4 個不重疊的子集合，比 (總和, index 字典序)
    fn brute(intervals: &[Vec<i32>]) -> Vec<i32> {
        let n = intervals.len();
        let mut best: (i64, Vec<i32>) = (0, Vec::new());
        for mask in 0u32..(1 << n) {
            if mask.count_ones() > 4 {
                continue;
            }
            let mut chosen: Vec<usize> = (0..n).filter(|&i| mask >> i & 1 == 1).collect();
            chosen.sort_by_key(|&i| (intervals[i][0], intervals[i][1]));
            if !chosen
                .windows(2)
                .all(|w| intervals[w[0]][1] < intervals[w[1]][0])
            {
                continue;
            }
            let sum: i64 = chosen.iter().map(|&i| intervals[i][2] as i64).sum();
            let mut ids: Vec<i32> = chosen.iter().map(|&i| i as i32).collect();
            ids.sort();
            if sum > best.0 || (sum == best.0 && ids < best.1) {
                best = (sum, ids);
            }
        }
        best.1
    }

    fn xorshift(seed: &mut u64) -> u64 {
        *seed ^= *seed << 13;
        *seed ^= *seed >> 7;
        *seed ^= *seed << 17;
        *seed
    }

    #[test]
    fn random_vs_brute() {
        let mut seed: u64 = 88172645463325252;
        for _ in 0..500 {
            let n = 1 + (xorshift(&mut seed) % 8) as usize;
            let intervals: Vec<Vec<i32>> = (0..n)
                .map(|_| {
                    let l = 1 + (xorshift(&mut seed) % 6) as i32;
                    let r = l + (xorshift(&mut seed) % 4) as i32;
                    let w = 1 + (xorshift(&mut seed) % 10) as i32;
                    vec![l, r, w]
                })
                .collect();

            let want = brute(&intervals);
            let got = Solution::maximum_weight(intervals.clone());
            assert_eq!(got, want, "intervals={intervals:?}");
        }
    }
}
