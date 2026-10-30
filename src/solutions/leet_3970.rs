use std::{cmp::Reverse, collections::BinaryHeap};

struct Solution {}

impl Solution {
    pub fn shortest_path(n: i32, edges: Vec<Vec<i32>>, labels: String, k: i32) -> i32 {
        let n = n as usize;
        let neighbors = Self::build_neighbors(n, &edges);
        let bytes = labels.into_bytes();

        // (path,node,parent,remain_k)
        let mut heap: BinaryHeap<Reverse<(i32, usize, i32)>> = BinaryHeap::new();
        let mut min_paths = vec![vec![i32::MAX; k as usize + 1]; n];
        heap.push(Reverse((0, 0usize, k - 1)));
        min_paths[0].iter_mut().for_each(|x| *x = 0);

        while let Some(Reverse((path, node, remain_k))) = heap.pop() {
            if path > min_paths[node][remain_k as usize] {
                continue;
            }

            if node == n - 1 {
                return path;
            }

            for &(next, weight) in neighbors[node].iter() {
                let next_k = if bytes[next] == bytes[node] {
                    remain_k - 1
                } else {
                    k - 1
                };

                if next_k < 0 {
                    continue;
                }

                let new_path = path + weight;
                if min_paths[next][next_k as usize] <= new_path {
                    continue;
                }

                min_paths[next][next_k as usize] = new_path;
                heap.push(Reverse((new_path, next, next_k)));
            }
        }

        -1
    }

    fn build_neighbors(n: usize, edges: &[Vec<i32>]) -> Vec<Vec<(usize, i32)>> {
        let mut neighbors = vec![Vec::new(); n];
        for edge in edges {
            let left = edge[0] as usize;
            let right = edge[1] as usize;
            let weight = edge[2];
            neighbors[left].push((right, weight));
            // neighbors[right].push((left, weight));
        }

        return neighbors;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::shortest_path(
                3,
                vec![vec![0, 1, 1], vec![1, 2, 1], vec![0, 2, 3]],
                "aab".to_string(),
                1
            ),
            3
        );
    }
}
