struct Solution {}

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();
        let mut visited: Vec<bool> = vec![false; n];
        let mut group = 0;
        let mut stack = Vec::new();

        for start in 0..n {
            if visited[start] {
                continue;
            }

            group += 1;
            visited[start] = true;
            stack.push(start);

            while let Some(curr) = stack.pop() {
                for next in 0..n {
                    if is_connected[curr][next] == 1 && !visited[next] {
                        visited[next] = true;
                        stack.push(next);
                    }
                }
            }
        }

        group
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::find_circle_num(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]]),
            2
        )
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::find_circle_num(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]),
            3
        )
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::find_circle_num(vec![vec![1]]), 1)
    }

    #[test]
    fn chain() {
        assert_eq!(
            Solution::find_circle_num(vec![
                vec![1, 1, 0, 0],
                vec![1, 1, 1, 0],
                vec![0, 1, 1, 0],
                vec![0, 0, 0, 1],
            ]),
            2
        )
    }
}
