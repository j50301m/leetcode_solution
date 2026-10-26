struct Solution {}

impl Solution {
    pub fn count_islands(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let k = k as i64;
        let mut visited = vec![vec![false; n]; m];
        let mut count = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] != 0 && !visited[i][j] {
                    if Self::dfs(&grid, &mut visited, (i, j)) % k == 0 {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    fn dfs(grid: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, pos: (usize, usize)) -> i64 {
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;

        let (row, col) = (pos.0, pos.1);
        visited[row][col] = true;

        let mut sum = grid[row][col] as i64;
        for (dr, dc) in [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
            let r = row as i32 + dr;
            let c = col as i32 + dc;

            if r < 0 || r > m - 1 || c < 0 || c > n - 1 {
                continue;
            }

            let r = r as usize;
            let c = c as usize;
            if grid[r][c] == 0 || visited[r][c] {
                continue;
            }

            sum += Self::dfs(grid, visited, (r, c));
        }

        sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        let grid = vec![
            vec![0, 2, 1, 0, 0],
            vec![0, 5, 0, 0, 5],
            vec![0, 0, 1, 0, 0],
            vec![0, 1, 4, 7, 0],
            vec![0, 2, 0, 0, 8],
        ];
        assert_eq!(Solution::count_islands(grid, 5), 2);
    }

    #[test]
    fn example_2() {
        let grid = vec![vec![3, 0, 3, 0], vec![0, 3, 0, 3], vec![3, 0, 3, 0]];
        assert_eq!(Solution::count_islands(grid, 3), 6);
    }

    // 非正方形：列數 != 行數，迴圈邊界要分開用 m 和 n
    #[test]
    fn wide_grid() {
        let grid = vec![vec![1, 1, 0, 2, 3, 0, 4]];
        assert_eq!(Solution::count_islands(grid, 2), 2);
    }

    // 同一座島只能算一次：已走過的格子不能再當起點、DFS 也不能走回頭
    #[test]
    fn one_big_island() {
        let grid = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];
        assert_eq!(Solution::count_islands(grid, 9), 1);
        let grid = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];
        assert_eq!(Solution::count_islands(grid, 3), 1);
    }

    // 往左右走的島（直的島測不出 col 方向有沒有寫對）
    #[test]
    fn horizontal_island() {
        let grid = vec![vec![0, 0, 0], vec![2, 3, 5], vec![0, 0, 0]];
        assert_eq!(Solution::count_islands(grid, 10), 1);
        let grid = vec![vec![0, 0, 0], vec![2, 3, 5], vec![0, 0, 0]];
        assert_eq!(Solution::count_islands(grid, 7), 0);
    }

    #[test]
    fn all_water() {
        assert_eq!(Solution::count_islands(vec![vec![0, 0], vec![0, 0]], 1), 0);
    }

    // 3000 * 1_000_000 = 3e9 超過 i32 上限
    #[test]
    fn large_sum() {
        let grid = vec![vec![1_000_000; 3000]];
        assert_eq!(Solution::count_islands(grid, 1_000_000), 1);
    }
}
