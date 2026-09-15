use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut visited = vec![vec![false; n]; m];
        let mut queue = VecDeque::new();
        let mut freash_cnt = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 2 {
                    queue.push_front((i, j, 0));
                    visited[i][j] = true;
                } else if grid[i][j] == 1 {
                    freash_cnt += 1;
                }
            }
        }

        let mut max_step = 0;
        while let Some((i, j, step)) = queue.pop_back() {
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let x = i as i32 + dx;
                let y = j as i32 + dy;

                // Check the boundary
                if x < 0 || x >= m as i32 || y < 0 || y >= n as i32 {
                    continue;
                }

                let x = x as usize;
                let y = y as usize;
                // Only fresh orange can be rotten
                if grid[x][y] != 1 {
                    continue;
                }

                if visited[x][y] {
                    continue;
                }

                visited[x][y] = true;
                queue.push_front((x, y, step + 1));
                max_step = max_step.max(step + 1);
                freash_cnt -= 1;
            }
        }

        if freash_cnt > 0 {
            return -1;
        }

        max_step
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn grid(rows: &[&[i32]]) -> Vec<Vec<i32>> {
        rows.iter().map(|r| r.to_vec()).collect()
    }

    #[test]
    fn case1() {
        assert_eq!(
            Solution::oranges_rotting(grid(&[&[2, 1, 1], &[1, 1, 0], &[0, 1, 1]])),
            4
        )
    }

    #[test]
    fn unreachable_fresh_orange() {
        assert_eq!(
            Solution::oranges_rotting(grid(&[&[2, 1, 1], &[0, 1, 1], &[1, 0, 1]])),
            -1
        )
    }

    #[test]
    fn no_fresh_orange() {
        assert_eq!(Solution::oranges_rotting(grid(&[&[0, 2]])), 0)
    }

    #[test]
    fn fresh_without_any_rotten() {
        assert_eq!(Solution::oranges_rotting(grid(&[&[1]])), -1)
    }

    #[test]
    fn empty_cells_only() {
        assert_eq!(Solution::oranges_rotting(grid(&[&[0, 0], &[0, 0]])), 0)
    }

    #[test]
    fn multiple_sources_spread_together() {
        assert_eq!(Solution::oranges_rotting(grid(&[&[2, 1, 1, 1, 2]])), 2)
    }

    #[test]
    fn two_sources_reach_same_orange() {
        assert_eq!(Solution::oranges_rotting(grid(&[&[2, 1, 2]])), 1)
    }
}
