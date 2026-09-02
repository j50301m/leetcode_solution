use std::collections::{HashMap, VecDeque};

struct Solution {}

impl Solution {
    pub fn min_moves(classroom: Vec<String>, energy: i32) -> i32 {
        let grid: Vec<&[u8]> = classroom.iter().map(|s| s.as_bytes()).collect();
        let (m, n) = (grid.len(), grid[0].len());

        // 第一步 找到start,和litter位置圖
        let (mut start_row, mut start_col) = (0, 0);
        let mut litter_pos: HashMap<(usize, usize), usize> = HashMap::new(); // key:litter pos, value: litter order
        let mut k = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == b'S' {
                    start_row = i;
                    start_col = j;
                } else if grid[i][j] == b'L' {
                    litter_pos.insert((i, j), k);
                    k += 1;
                }
            }
        }

        // 如果沒有litter 直接回傳
        if k == 0 {
            return 0;
        }

        let full: i32 = (1 << k) - 1;
        const DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        let mut best = vec![vec![vec![-1i32; 1 << k]; n]; m]; // [row][col][mask] val =energy;

        let mut queue: VecDeque<((usize, usize), i32, i32, i32)> = VecDeque::new(); // (curr_pos, energy, mask,curr_step)
        queue.push_back(((start_row, start_col), energy, 0, 0));
        while let Some(((r, c), e, mask, step)) = queue.pop_front() {
            if e == 0 {
                continue;
            }

            for (dr, dc) in DIRS {
                let (nr, nc) = (r as i32 + dr, c as i32 + dc);
                // 如果遇到邊界 繼續下一步
                if nr < 0 || nc < 0 || nr >= m as i32 || nc >= n as i32 {
                    continue;
                }

                let (nr, nc) = (nr as usize, nc as usize);
                // 如果這一個不能走 繼續下一步
                if grid[nr][nc] == b'X' {
                    continue;
                }

                let mut ne = e - 1;
                let mut nmask = mask;
                // 如果這一格是Litter更新mask
                if let Some(order) = litter_pos.get(&(nr, nc)) {
                    nmask |= 1 << order;
                }

                // 如果這一格 是R 補滿能量
                if grid[nr][nc] == b'R' {
                    ne = energy;
                }

                // 如果已經找到所有litter early return.
                if nmask == full {
                    return step + 1;
                }

                // 剪枝，如果這個mask所剩能量 小於best內記錄的能量，不值得排入搜尋
                if ne > best[nr][nc][nmask as usize] {
                    best[nr][nc][nmask as usize] = ne;
                    queue.push_back(((nr, nc), ne, nmask, step + 1));
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn build(rows: &[&str]) -> Vec<String> {
        rows.iter().map(|s| s.to_string()).collect()
    }

    // 官方 example 1：正下方是 X，只能繞右邊
    #[test]
    fn case1() {
        assert_eq!(Solution::min_moves(build(&["S.", "XL"]), 2), 2);
    }

    // 官方 example 2：中途踩 R 把電補滿才走得完
    #[test]
    fn case2() {
        assert_eq!(Solution::min_moves(build(&["LS", "RL"]), 4), 3);
    }

    // 官方 example 3：2x3 非正方形 -> 抓 best[nc][nr] / push (nc, nr) 這種行列顛倒
    #[test]
    fn case3() {
        assert_eq!(Solution::min_moves(build(&["L.S", "RXL"]), 3), -1);
    }

    // 一步就撿完 -> 抓 return step 忘了 +1（忘了會回 0）
    #[test]
    fn case4() {
        assert_eq!(Solution::min_moves(build(&["SL"]), 1), 1);
    }

    // 走到 R 時能量剛好扣成 0，但踩在 R 上要補滿繼續
    // -> 抓「先判斷沒電就死」再看腳下的順序錯誤（順序錯會回 -1）
    #[test]
    fn case5() {
        assert_eq!(Solution::min_moves(build(&["SRL"]), 1), 2);
    }

    // 陷阱盤：(1,1) mask=01 先被一條沒電的路佔走
    // -> 抓剪枝寫成「來過就跳過」（寫錯會回 -1）
    #[test]
    fn case6() {
        assert_eq!(Solution::min_moves(build(&["SR.", ".LL"]), 2), 3);
    }

    // 反覆進出兩個 R 才撿得完，路徑比直覺長
    #[test]
    fn case7() {
        assert_eq!(Solution::min_moves(build(&["L.L", "SRR"]), 2), 7);
    }

    // 拿掉 R 就是死局 -> 確認 R 真的有被當成補滿而不是普通空地
    #[test]
    fn case8() {
        assert_eq!(Solution::min_moves(build(&["...", "SR.", "LXL"]), 3), 5);
    }

    // 垃圾被 X 完全隔開，queue 空掉 -> -1
    #[test]
    fn case9() {
        assert_eq!(Solution::min_moves(build(&["SX", "XL"]), 5), -1);
    }

    // 一個垃圾都沒有 -> 抓 k == 0 沒擋（沒擋會跑完 BFS 回 -1）
    #[test]
    fn case10() {
        assert_eq!(Solution::min_moves(build(&["S"]), 1), 0);
    }

    // 4 個垃圾 -> 抓 full 或 best 的 1 << k 開錯寬度
    #[test]
    fn case11() {
        assert_eq!(Solution::min_moves(build(&["LLLL", "S..."]), 10), 4);
    }
}
