use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

struct Solution {}

impl Solution {
    pub fn get_food_dijkstra(grid: Vec<Vec<char>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut start = (-1, -1);
        for (row_idx, row) in grid.iter().enumerate() {
            let mut is_found = false;
            for (col_idx, cell) in row.iter().enumerate() {
                if *cell == '*' {
                    start = (row_idx as i32, col_idx as i32);
                    is_found = true;
                    break;
                }
            }
            if is_found {
                break;
            }
        }

        let mut heap: BinaryHeap<Reverse<(i32, i32, i32)>> = BinaryHeap::new(); // (curr_path,row,col)
        let mut paths = vec![vec![i32::MAX; n]; m];
        paths[start.0 as usize][start.1 as usize] = 0;
        heap.push(Reverse((0, start.0, start.1)));
        while let Some(Reverse((curr_path, curr_r, curr_c))) = heap.pop() {
            if curr_path > paths[curr_r as usize][curr_c as usize] {
                continue;
            }

            if grid[curr_r as usize][curr_c as usize] == '#' {
                return curr_path;
            }

            for (dr, dc) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let r = curr_r + dr;
                let c = curr_c + dc;

                if r < 0 || r >= m as i32 || c < 0 || c >= n as i32 {
                    continue;
                }

                match grid[r as usize][c as usize] {
                    'X' | '*' => continue,
                    'O' | '#' => {
                        let path = curr_path + 1;
                        if paths[r as usize][c as usize] <= path {
                            continue;
                        }

                        paths[r as usize][c as usize] = path;
                        heap.push(Reverse((path, r, c)));
                    }
                    _ => continue, // unreachable
                }
            }
        }

        -1
    }

    pub fn get_food(grid: Vec<Vec<char>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut start = (-1, -1);
        for (row_idx, row) in grid.iter().enumerate() {
            let mut is_found = false;
            for (col_idx, cell) in row.iter().enumerate() {
                if *cell == '*' {
                    start = (row_idx as i32, col_idx as i32);
                    is_found = true;
                    break;
                }
            }
            if is_found {
                break;
            }
        }

        let mut queue = VecDeque::new();
        let mut visited = vec![vec![false; n]; m];
        queue.push_front((0, start.0, start.1));
        visited[start.0 as usize][start.1 as usize] = true;
        while let Some((curr_path, curr_r, curr_c)) = queue.pop_back() {
            for (dr, dc) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let r = curr_r + dr;
                let c = curr_c + dc;

                if r < 0 || r >= m as i32 || c < 0 || c >= n as i32 {
                    continue;
                }

                match grid[r as usize][c as usize] {
                    '#' => return curr_path + 1,
                    'X' | '*' => continue,
                    'O' => {
                        if visited[r as usize][c as usize] {
                            continue;
                        }
                        visited[r as usize][c as usize] = true;
                        queue.push_front((curr_path + 1, r, c));
                    }
                    _ => continue, // unreachable
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn to_grid(rows: &[&str]) -> Vec<Vec<char>> {
        rows.iter().map(|row| row.chars().collect()).collect()
    }

    #[test]
    fn case1() {
        let grid = to_grid(&["XXXXXX", "X*OOOX", "XOO#OX", "XXXXXX"]);
        assert_eq!(Solution::get_food(grid), 3);
    }

    #[test]
    fn case2() {
        let grid = to_grid(&["XXXXX", "X*XOX", "XOX#X", "XXXXX"]);
        assert_eq!(Solution::get_food(grid), -1);
    }

    #[test]
    fn case3() {
        let grid = to_grid(&["XXXXXXXX", "X*OXO#OX", "XOOXOOXX", "XOOOO#OX", "XXXXXXXX"]);
        assert_eq!(Solution::get_food(grid), 6);
    }

    #[test]
    fn case4() {
        let grid = to_grid(&["O*", "#O"]);
        assert_eq!(Solution::get_food(grid), 2);
    }

    #[test]
    fn case5_blocked_by_walls() {
        let grid = to_grid(&["X*", "#X"]);
        assert_eq!(Solution::get_food(grid), -1);
    }

    #[test]
    fn case6_food_adjacent() {
        let grid = to_grid(&["*#"]);
        assert_eq!(Solution::get_food(grid), 1);
    }

    #[test]
    fn case7_single_cell() {
        let grid = to_grid(&["*"]);
        assert_eq!(Solution::get_food(grid), -1);
    }

    #[test]
    fn case8_no_food_open_area() {
        let grid = to_grid(&["*OO", "OOO"]);
        assert_eq!(Solution::get_food(grid), -1);
    }
}
