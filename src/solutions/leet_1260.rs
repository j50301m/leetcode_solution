struct Solution;

impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let k = k as usize;
        let mut new_grid = vec![vec![0; n]; m];

        let index_to_pos = |i: usize, j: usize| -> (usize, usize) {
            let index = i * n + j;
            let new_index = (index + k) % (m * n);
            (new_index / n, new_index % n)
        };

        for i in 0..m {
            for j in 0..n {
                let (new_i, new_j) = index_to_pos(i, j);
                new_grid[new_i][new_j] = grid[i][j];
            }
        }
        new_grid
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![vec![9, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
        assert_eq!(Solution::shift_grid(grid, 1), expected);
    }

    #[test]
    fn case_2() {
        let grid = vec![
            vec![3, 8, 1, 9],
            vec![19, 7, 2, 5],
            vec![4, 6, 11, 10],
            vec![12, 0, 21, 13],
        ];
        let expected = vec![
            vec![12, 0, 21, 13],
            vec![3, 8, 1, 9],
            vec![19, 7, 2, 5],
            vec![4, 6, 11, 10],
        ];
        assert_eq!(Solution::shift_grid(grid, 4), expected);
    }

    #[test]
    fn k_equals_full_cycle() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(Solution::shift_grid(grid.clone(), 9), grid);
    }
}
