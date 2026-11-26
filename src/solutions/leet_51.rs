struct Solution {}

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;

        let mut ans = Vec::new();
        let mut board = vec![vec!['.'; n]; n];
        Self::dfs(0, n, &mut board, &mut ans);

        ans
    }

    fn dfs(
        curr_layer: usize,
        target_layer: usize,
        board: &mut Vec<Vec<char>>,
        ans: &mut Vec<Vec<String>>,
    ) {
        if curr_layer == target_layer {
            let mut collect = Vec::new();
            for chars in board.iter() {
                let str = chars
                    .iter()
                    .map(|&c| if c == 'Q' { 'Q' } else { '.' })
                    .collect::<String>();
                collect.push(str);
            }
            ans.push(collect);
            return;
        }

        let n = board.len();
        for i in 0..n {
            if board[curr_layer][i] == '.' && Self::verify(curr_layer, i, board) {
                Self::update(curr_layer, i, board);
                Self::dfs(curr_layer + 1, target_layer, board, ans);
                Self::rollback(curr_layer, i, board);
            }
        }
    }

    fn verify(row: usize, col: usize, board: &Vec<Vec<char>>) -> bool {
        let n = board.len();
        for i in 0..n {
            if board[row][i] == 'Q' || board[i][col] == 'Q' {
                return false;
            }

            if row >= i && col >= i {
                if board[row - i][col - i] == 'Q' {
                    return false;
                }
            }

            if row + i < n && col >= i {
                if board[row + i][col - i] == 'Q' {
                    return false;
                }
            }

            if row >= i && col + i < n {
                if board[row - i][col + i] == 'Q' {
                    return false;
                }
            }

            if row + i < n && col + i < n {
                if board[row + i][col + i] == 'Q' {
                    return false;
                }
            }
        }

        true
    }

    fn update(row: usize, col: usize, board: &mut Vec<Vec<char>>) {
        let n = board.len();
        for i in 0..n {
            board[row][i] = '_';
            board[i][col] = '_';

            if row >= i && col >= i {
                board[row - i][col - i] = '_';
            }

            if row + i < n && col >= i {
                board[row + i][col - i] = '_';
            }

            if row >= i && col + i < n {
                board[row - i][col + i] = '_';
            }

            if row + i < n && col + i < n {
                board[row + i][col + i] = '_';
            }
        }
        board[row][col] = 'Q';
    }

    fn rollback(row: usize, col: usize, board: &mut Vec<Vec<char>>) {
        let n = board.len();
        for i in 0..n {
            board[row][i] = '.';
            board[i][col] = '.';

            if row >= i && col >= i {
                board[row - i][col - i] = '.';
            }

            if row + i < n && col >= i {
                board[row + i][col - i] = '.';
            }

            if row >= i && col + i < n {
                board[row - i][col + i] = '.';
            }

            if row + i < n && col + i < n {
                board[row + i][col + i] = '.';
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::solve_n_queens(4),
            vec![
                vec![".Q..", "...Q", "Q...", "..Q."],
                vec!["..Q.", "Q...", "...Q", ".Q.."],
            ]
        );
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::solve_n_queens(1), vec![vec!["Q"]]);
    }

    #[test]
    fn no_solution() {
        assert!(Solution::solve_n_queens(2).is_empty());
        assert!(Solution::solve_n_queens(3).is_empty());
    }

    #[test]
    fn count_only() {
        assert_eq!(Solution::solve_n_queens(5).len(), 10);
        assert_eq!(Solution::solve_n_queens(8).len(), 92);
    }
}
