use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

struct Solution {}

impl Solution {
    pub fn minimum_cost_dfs(n: i32, highways: Vec<Vec<i32>>, discounts: i32) -> i32 {
        let mut visited = vec![vec![usize::MAX; (discounts + 1) as usize]; n as usize];
        let mut map: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
        for item in highways {
            let n1 = item[0] as usize;
            let n2 = item[1] as usize;
            let dist = item[2] as usize;

            let vec = map.entry(n1).or_insert(Vec::new());
            vec.push((n2, dist));

            let vec = map.entry(n2).or_insert(Vec::new());
            vec.push((n1, dist));
        }

        let result = Self::dfs((n - 1) as usize, 0, 0, &map, &mut visited, discounts);
        if result == usize::MAX {
            return -1;
        }
        result as i32
    }

    fn dfs(
        target: usize,
        curr: usize,
        curr_dist: usize,
        map: &HashMap<usize, Vec<(usize, usize)>>,
        visited: &mut Vec<Vec<usize>>,
        discounts: i32,
    ) -> usize {
        if curr == target {
            return curr_dist;
        }

        let mut target_dist = usize::MAX;
        let Some(node) = map.get(&curr) else {
            return target_dist;
        };
        for &(next_node, dist) in node {
            if discounts > 0 {
                let d = curr_dist + dist / 2;
                let min_dist = visited[next_node][(discounts - 1) as usize..]
                    .iter()
                    .min()
                    .unwrap();
                if d < *min_dist {
                    visited[next_node][(discounts - 1) as usize] = d;
                    target_dist = target_dist.min(Self::dfs(
                        target,
                        next_node,
                        d,
                        map,
                        visited,
                        discounts - 1,
                    ));
                }
            }
            let d = curr_dist + dist;
            let min_dist = visited[next_node][discounts as usize..]
                .iter()
                .min()
                .unwrap();
            if d < *min_dist {
                visited[next_node][discounts as usize] = d;
                target_dist =
                    target_dist.min(Self::dfs(target, next_node, d, map, visited, discounts));
            }
        }

        target_dist
    }

    // min_cost[0][discounts] = 0;
    // heap.push(Reverse((0, 0, discounts)));
    // while let Some(Reverse((cost, city, discounts_left))) = heap.pop() {
    //     if cost > min_cost[city][discounts_left] { continue; }   // 過期
    //     if city == n - 1 { return cost; }   // 第一次到終點 = 答案
    //     for &(next_city, toll) in &roads[city] {
    //         // 原價：留在同一層
    //         try_update(next_city, discounts_left, cost + toll);
    //         // 打折：掉到下一層，少一次折扣
    //         if discounts_left > 0 {
    //             try_update(next_city, discounts_left - 1, cost + toll / 2);
    //         }
    //     }
    // }
    // return -1;
    pub fn minimum_cost(n: i32, highways: Vec<Vec<i32>>, discounts: i32) -> i32 {
        let discounts = discounts as usize;
        let n = n as usize;
        let mut min_cost = vec![vec![usize::MAX; discounts + 1]; n];
        let mut heap = BinaryHeap::new();

        // Build map
        let mut map: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];
        for item in highways {
            let n1 = item[0] as usize;
            let n2 = item[1] as usize;
            let toll = item[2] as usize;

            map[n1].push((n2, toll));
            map[n2].push((n1, toll));
        }

        // Initialize
        min_cost[0][discounts] = 0;
        heap.push(Reverse((0, 0, discounts)));
        while let Some(Reverse((cost, city, discount_left))) = heap.pop() {
            if cost > min_cost[city][discount_left] {
                continue;
            }
            if city == n - 1 {
                return cost as i32;
            }

            for &(next_city, toll) in map[city].iter() {
                if discount_left > 0 {
                    let new_cost = cost + toll / 2;
                    if min_cost[next_city][discount_left - 1] > new_cost {
                        min_cost[next_city][discount_left - 1] = new_cost;
                        heap.push(Reverse((new_cost, next_city, discount_left - 1)));
                    }
                }

                let new_cost = cost + toll;
                if min_cost[next_city][discount_left] > new_cost {
                    min_cost[next_city][discount_left] = new_cost;
                    heap.push(Reverse((new_cost, next_city, discount_left)));
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::minimum_cost(
                5,
                vec![
                    vec![0, 1, 4],
                    vec![2, 1, 3],
                    vec![1, 4, 11],
                    vec![3, 2, 3],
                    vec![3, 4, 2]
                ],
                1
            ),
            9
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::minimum_cost(
                4,
                vec![
                    vec![1, 3, 17],
                    vec![1, 2, 7],
                    vec![3, 2, 5],
                    vec![0, 1, 6],
                    vec![3, 0, 20]
                ],
                20
            ),
            8
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            Solution::minimum_cost(4, vec![vec![0, 1, 3], vec![2, 3, 2]], 0),
            -1
        );
    }

    // node 0 has no highways at all
    #[test]
    fn case4() {
        assert_eq!(Solution::minimum_cost(3, vec![vec![1, 2, 5]], 1), -1);
    }

    // best: 0->2 discounted (2) + 2->3 discounted (50) = 52
    #[test]
    fn case5() {
        assert_eq!(
            Solution::minimum_cost(
                4,
                vec![vec![0, 1, 2], vec![1, 2, 2], vec![0, 2, 5], vec![2, 3, 100]],
                2
            ),
            52
        );
    }
}
