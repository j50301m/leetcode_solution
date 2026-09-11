use std::collections::{HashMap, VecDeque};

struct Solution {}

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut map = HashMap::new();
        for equation in equations.iter() {
            for val in equation.iter() {
                let next = map.len();
                map.entry(val).or_insert(next);
            }
        }

        let mut roads = vec![Vec::new(); map.len()];
        for i in 0..equations.len() {
            let (a, b) = (&equations[i][0], &equations[i][1]);
            let (&a_idx, &b_idx) = (map.get(a).unwrap(), map.get(b).unwrap());
            let value = values[i];
            roads[a_idx].push((b_idx, value));
            roads[b_idx].push((a_idx, 1f64 / value));
        }

        let mut results = Vec::new();
        for query in queries.into_iter() {
            let (a, b) = (&query[0], &query[1]);
            let (Some(&a), Some(&b)) = (map.get(&a), map.get(&b)) else {
                results.push(-1f64);
                continue;
            };

            let ans = Self::bfs(&roads, a, b);
            results.push(ans);
        }
        results
    }

    fn bfs(roads: &Vec<Vec<(usize, f64)>>, a: usize, b: usize) -> f64 {
        let mut queue = VecDeque::from([(a, 1f64)]);
        let mut visited = vec![false; roads.len()];
        while let Some((curr, curr_val)) = queue.pop_front() {
            if visited[curr] {
                continue;
            }
            visited[curr] = true;
            if curr == b {
                return curr_val;
            }
            for &(target, val) in roads[curr].iter() {
                queue.push_back((target, curr_val * val));
            }
        }
        -1f64
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn pairs(p: &[[&str; 2]]) -> Vec<Vec<String>> {
        p.iter()
            .map(|[a, b]| vec![a.to_string(), b.to_string()])
            .collect()
    }

    // ponytail: 浮點數不能用 assert_eq!，LeetCode 容許 1e-5 誤差
    fn check(eq: &[[&str; 2]], values: &[f64], q: &[[&str; 2]], want: &[f64]) {
        let got = Solution::calc_equation(pairs(eq), values.to_vec(), pairs(q));
        assert_eq!(got.len(), want.len(), "結果數量不對\n got: {got:?}");
        for (g, w) in got.iter().zip(want) {
            assert!((g - w).abs() < 1e-5, "\n got:  {got:?}\n want: {want:?}");
        }
    }

    #[test]
    fn case1() {
        check(
            &[["a", "b"], ["b", "c"]],
            &[2.0, 3.0],
            &[["a", "c"], ["b", "a"], ["a", "e"], ["a", "a"], ["x", "x"]],
            &[6.0, 0.5, -1.0, 1.0, -1.0],
        )
    }

    #[test]
    fn case2() {
        check(
            &[["a", "b"], ["b", "c"], ["bc", "cd"]],
            &[1.5, 2.5, 5.0],
            &[["a", "c"], ["c", "b"], ["bc", "cd"], ["cd", "bc"]],
            &[3.75, 0.4, 5.0, 0.2],
        )
    }

    #[test]
    fn case3() {
        check(
            &[["a", "b"]],
            &[0.5],
            &[["a", "b"], ["b", "a"], ["a", "c"], ["x", "y"]],
            &[0.5, 2.0, -1.0, -1.0],
        )
    }

    /// 兩個互不相連的群組，跨群組查詢要回 -1
    #[test]
    fn disconnected() {
        check(
            &[["a", "b"], ["c", "d"]],
            &[2.0, 3.0],
            &[["a", "d"], ["a", "b"], ["d", "c"]],
            &[-1.0, 2.0, 1.0 / 3.0],
        )
    }

    /// 四跳的長路徑，確認 BFS 有跟著 curr 往外擴
    #[test]
    fn long_path() {
        check(
            &[["a", "b"], ["b", "c"], ["c", "d"], ["d", "e"]],
            &[2.0, 2.0, 2.0, 2.0],
            &[["a", "e"], ["e", "a"]],
            &[16.0, 0.0625],
        )
    }

    /// x/x 只要 x 存在就是 1.0，不存在才是 -1.0
    #[test]
    fn self_division() {
        check(
            &[["a", "b"]],
            &[2.0],
            &[["a", "a"], ["b", "b"], ["z", "z"]],
            &[1.0, 1.0, -1.0],
        )
    }
}
