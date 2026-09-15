use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let n = maze.len() as i32;
        let m = maze[0].len() as i32;
        let mut visited = vec![vec![false; m as usize]; n as usize];
        let mut queue = VecDeque::new();
        queue.push_front(((entrance[0], entrance[1]), 0));
        while let Some(((curr_r, curr_c), step)) = queue.pop_back() {
            visited[curr_r as usize][curr_c as usize] = true;
            for (dr, dc) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let r = curr_r + dr;
                let c = curr_c + dc;
                // Out of boundary
                if r < 0 || r >= n || c < 0 || c >= m {
                    if curr_r == entrance[0] && curr_c == entrance[1] {
                        continue;
                    } else {
                        return step;
                    }
                }

                // Run in hinder
                if maze[r as usize][c as usize] == '+' {
                    continue;
                }
                if visited[r as usize][c as usize] {
                    continue;
                }
                visited[r as usize][c as usize] = true;
                queue.push_front(((r, c), step + 1));
            }
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn maze(rows: &[&str]) -> Vec<Vec<char>> {
        rows.iter().map(|r| r.chars().collect()).collect()
    }

    #[test]
    fn case1() {
        assert_eq!(
            Solution::nearest_exit(maze(&["++.+", "...+", "+++."]), vec![1, 2]),
            1
        )
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::nearest_exit(maze(&[".+", ".."]), vec![1, 0]), 1)
    }

    #[test]
    fn no_exit() {
        assert_eq!(
            Solution::nearest_exit(maze(&["+++++", "+...+", "+++++"]), vec![1, 1]),
            -1
        )
    }

    #[test]
    fn entrance_itself_is_not_an_exit() {
        assert_eq!(Solution::nearest_exit(maze(&["."]), vec![0, 0]), -1)
    }

    #[test]
    fn case3() {
        assert_eq!(
            Solution::nearest_exit(
                maze(&["+++++", "+...+", "+.+.+", "+...+", "++.++"]),
                vec![1, 1]
            ),
            4
        )
    }
}
