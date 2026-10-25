struct Solution {}

impl Solution {
    pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut visited = vec![vec![false; n]; m];
        let mut count = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 && !visited[i][j] {
                    visited[i][j] = true;
                    if Self::dfs(&grid, &mut visited, (i, j)) {
                        count += 1;
                    }
                } else {
                    visited[i][j] = true;
                }
            }
        }

        count
    }

    fn dfs(grid: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, start: (usize, usize)) -> bool {
        let (row, col) = (start.0, start.1);
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;

        let mut is_closed = true;
        for (dr, dc) in [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)] {
            let r = row as i32 + dr;
            let c = col as i32 + dc;

            if r < 0 || r > m - 1 || c < 0 || c > n - 1 {
                is_closed = false;
                continue;
            }

            let r = r as usize;
            let c = c as usize;
            if visited[r][c] {
                continue;
            }

            visited[r][c] = true;
            if grid[r][c] == 1 {
                continue;
            }

            is_closed = Self::dfs(grid, visited, (r, c)) && is_closed;
        }

        is_closed
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let grid = vec![
            vec![1, 1, 1, 1, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0, 1, 1, 0],
            vec![1, 0, 1, 0, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0, 1, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 0],
        ];
        assert_eq!(Solution::closed_island(grid), 2);
    }

    #[test]
    fn case2() {
        let grid = vec![
            vec![0, 0, 1, 0, 0],
            vec![0, 1, 0, 1, 0],
            vec![0, 1, 1, 1, 0],
        ];
        assert_eq!(Solution::closed_island(grid), 1);
    }

    #[test]
    fn case3() {
        let grid = vec![
            vec![1, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 1, 1, 1, 0, 1],
            vec![1, 0, 1, 0, 1, 0, 1],
            vec![1, 0, 1, 1, 1, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1],
        ];
        assert_eq!(Solution::closed_island(grid), 2);
    }

    // 十字形：DFS 必須四個方向都走，不然左右兩格會被當成新島
    #[test]
    fn cross_shape() {
        let grid = vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 0, 1, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 1, 0, 1, 1],
            vec![1, 1, 1, 1, 1],
        ];
        assert_eq!(Solution::closed_island(grid), 1);
    }

    // 往下先碰到底邊，(1,2) 不能因為已經是 false 就不走
    #[test]
    fn touch_border_then_other_branch() {
        let grid = vec![
            vec![1, 1, 1, 1],
            vec![1, 0, 0, 1],
            vec![1, 0, 1, 1],
            vec![1, 0, 1, 1],
        ];
        assert_eq!(Solution::closed_island(grid), 0);
    }

    #[test]
    fn single_cell() {
        assert_eq!(Solution::closed_island(vec![vec![0]]), 0);
        assert_eq!(Solution::closed_island(vec![vec![1]]), 0);
    }
}
