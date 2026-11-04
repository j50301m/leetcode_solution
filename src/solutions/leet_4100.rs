struct Solution {}

impl Solution {
    pub fn min_activation_cost(edges: Vec<Vec<i32>>, values: Vec<i32>) -> i32 {
        let n = values.len();
        let mut neighbors: Vec<Vec<(i32, usize)>> = vec![Vec::new(); n]; // (cost,node)
        let mut parity = vec![0; n];
        for edge in edges.iter() {
            let (a, b, cost) = (edge[0] as usize, edge[1] as usize, edge[2]);
            let (val_a, val_b) = (values[a], values[b]);
            parity[a] = val_a.count_ones();
            parity[b] = val_b.count_ones();
            neighbors[a].push((cost, b));
            neighbors[b].push((cost, a));
        }

        let mut min_cost = i64::MAX;
        for edge in edges.iter() {
            let (a, b, cost) = (edge[0] as usize, edge[1] as usize, edge[2]);
            if (parity[a] + parity[b]) % 2 == 0 {
                min_cost = min_cost.min(cost as i64);
            }
        }

        for i in 0..n {
            if parity[i] % 2 != 0 {
                continue;
            }
            let mut min1 = i64::MAX;
            let mut min2 = i64::MAX;
            for &(cost, node) in neighbors[i].iter() {
                let cost = cost as i64;
                if parity[node] % 2 != 1 {
                    continue;
                }

                if cost < min1 {
                    min2 = min1;
                    min1 = cost;
                } else if cost < min2 {
                    min2 = cost;
                }
            }

            if min2 != i64::MAX {
                min_cost = min_cost.min(min1 + min2);
            }
        }

        if min_cost == i64::MAX {
            return -1;
        }
        min_cost as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::min_activation_cost(
                vec![vec![0, 1, 4], vec![0, 2, 6], vec![0, 3, 3], vec![3, 4, 5]],
                vec![3, 1, 7, 1, 3]
            ),
            7
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::min_activation_cost(vec![vec![0, 1, 5], vec![1, 2, 10]], vec![1, 7, 3]),
            5
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            Solution::min_activation_cost(
                vec![vec![0, 1, 5], vec![0, 2, 7], vec![0, 3, 9]],
                vec![1, 3, 5, 9]
            ),
            -1
        );
    }

    #[test]
    fn walking_back_is_not_a_path() {
        assert_eq!(
            Solution::min_activation_cost(vec![vec![0, 1, 5]], vec![1, 3]),
            -1
        );
    }

    #[test]
    fn single_edge_beats_long_edge() {
        assert_eq!(
            Solution::min_activation_cost(vec![vec![0, 1, 10], vec![1, 2, 1]], vec![1, 3, 3]),
            1
        );
    }
}
