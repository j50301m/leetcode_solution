struct Solution {}

impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());

        let mut row_paths = vec![Vec::new(); m];
        let mut col_paths = vec![Vec::new(); n];
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 {
                    continue;
                }
                row_paths[i].push(j);
                col_paths[j].push(i);
            }
        }

        let mut visited = vec![vec![false; n]; m];
        let mut count = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 || visited[i][j] == true {
                    continue;
                }

                visited[i][j] = true;
                if col_paths[j].len() == 1 && row_paths[i].len() == 1 {
                    continue;
                }

                count += 1;
                for &row in col_paths[j].iter() {
                    if visited[row][j] {
                        continue;
                    }

                    visited[row][j] = true;
                    count += 1;
                }

                for &col in row_paths[i].iter() {
                    if visited[i][col] {
                        continue;
                    }
                    visited[i][col] = true;
                    count += 1;
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::count_servers(vec![vec![1, 0], vec![0, 1]]), 0);
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::count_servers(vec![
                vec![1, 1, 0, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 0, 1]
            ]),
            4
        );
    }
}
