struct Solution {}

impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut roads = vec![Vec::new(); n];
        for connection in connections {
            let from = connection[0] as usize;
            let to = connection[1] as usize;
            roads[from].push((to, 1));
            roads[to].push((from, 0));
        }

        let mut visited = vec![false; n];
        let mut stack = Vec::new();
        let mut sum = 0;
        stack.push(0usize);
        while let Some(curr) = stack.pop() {
            visited[curr] = true;
            for &(next, way) in roads[curr].iter() {
                if visited[next] {
                    continue;
                }
                visited[next] = true;
                sum += way;
                stack.push(next);
            }
        }
        sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::min_reorder(
                6,
                vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]]
            ),
            3
        )
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::min_reorder(5, vec![vec![1, 0], vec![1, 2], vec![3, 2], vec![3, 4]]),
            2
        )
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::min_reorder(3, vec![vec![1, 0], vec![2, 0]]), 0)
    }
}
