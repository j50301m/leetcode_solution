struct Solution {}

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        let path_len = (m + n - 1) as i32;

        // 1. 路徑長度（m+n-1）是奇數則不可能到達
        // 2. 如果最後一個是'(' 則不可能
        if path_len % 2 != 0 || grid[m - 1][n - 1] == '(' || grid[0][0] == ')' {
            return false;
        }

        let mut stack = Vec::new();
        let mut visited = vec![vec![vec![false; (path_len + 1 / 2) as usize]; n]; m];
        stack.push((0usize, 0usize, 1, 1)); // [row,col,'('of cnt,step)]
        while let Some((row, col, cnt, step)) = stack.pop() {
            if row == m - 1 && col == n - 1 && cnt == 0 {
                return true;
            }

            for (dx, dy) in [(1usize, 0usize), (0usize, 1usize)] {
                let x = row + dx;
                let y = col + dy;
                if x > m - 1 || y > n - 1 {
                    continue;
                }

                let new_cnt = if grid[x][y] == '(' { cnt + 1 } else { cnt - 1 };
                if new_cnt > path_len - step || new_cnt < 0 || visited[x][y][new_cnt as usize] {
                    continue;
                }
                stack.push((x, y, new_cnt, step + 1));
                visited[x][y][new_cnt as usize] = true;
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn g(rows: &[&str]) -> Vec<Vec<char>> {
        rows.iter().map(|r| r.chars().collect()).collect()
    }

    #[test]
    fn case1() {
        assert!(Solution::has_valid_path(g(&["(((", ")()", "(()", "(()"])));
    }

    #[test]
    fn case2() {
        assert!(!Solution::has_valid_path(g(&["))", "(("])));
    }

    #[test]
    fn single_row() {
        assert!(Solution::has_valid_path(g(&["()"])));
    }

    #[test]
    fn odd_length() {
        assert!(!Solution::has_valid_path(g(&["(()"])));
    }

    // 兩個方向分岔時，第二個方向的 cnt 不能被第一個方向影響
    #[test]
    fn branch_cnt() {
        assert!(Solution::has_valid_path(g(&["()(", "(()"])));
    }

    // 中途 balance 變負數，即使最後回到 0 也不合法
    #[test]
    fn negative_midway() {
        assert!(!Solution::has_valid_path(g(&["())(()"])));
    }
}
