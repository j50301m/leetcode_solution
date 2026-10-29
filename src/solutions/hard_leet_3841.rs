struct Solution {}

const LOG: usize = 16; // 2^16 = 65536 > 5×10^4，深度差最多 16 個 bit
impl Solution {
    pub fn palindrome_path(
        n: i32,
        edges: Vec<Vec<i32>>,
        s: String,
        queries: Vec<String>,
    ) -> Vec<bool> {
        let (parent, depth, start, end) = Self::build_parent(n as usize, &edges);
        let ancestor_table = Self::build_ancestor_table(&parent);
        let mut chars = s.into_bytes();

        let mut bit = BIT::new(&chars, &start, &end);

        let mut ans = Vec::with_capacity(queries.len());
        for query in queries.iter() {
            let inputs = query.split(' ').collect::<Vec<&str>>();

            match inputs[0] {
                "update" => {
                    let node = inputs[1].parse::<usize>().unwrap();
                    let char = inputs[2].as_bytes()[0];
                    let change = BIT::char_to_i32(char) ^ BIT::char_to_i32(chars[node]);
                    bit.range_update(start[node], end[node], change);
                    chars[node] = char;
                }
                "query" => {
                    let node_a = inputs[1].parse::<usize>().unwrap();
                    let node_b = inputs[2].parse::<usize>().unwrap();
                    let lca = Self::lowest_common_ancestor(node_a, node_b, &ancestor_table, &depth);

                    let path_mask = bit.query(start[node_a])
                        ^ bit.query(start[node_b])
                        ^ BIT::char_to_i32(chars[lca]);
                    ans.push(path_mask.count_ones() <= 1);
                }
                _ => unreachable!(),
            }
        }

        ans
    }

    fn lowest_common_ancestor(
        mut node_a: usize,
        mut node_b: usize,
        ancestors: &[Vec<usize>],
        depth: &[usize],
    ) -> usize {
        if depth[node_b] > depth[node_a] {
            std::mem::swap(&mut node_a, &mut node_b);
        }
        let depth_gap = depth[node_a] - depth[node_b];
        for i in (0..LOG).rev() {
            if (depth_gap >> i) & 1 == 1 {
                node_a = ancestors[i][node_a];
            }
        }

        if node_a == node_b {
            return node_a;
        }

        for i in (0..LOG).rev() {
            if ancestors[i][node_a] != ancestors[i][node_b] {
                node_a = ancestors[i][node_a];
                node_b = ancestors[i][node_b];
            }
        }

        ancestors[0][node_a]
    }

    fn build_ancestor_table(parent: &[usize]) -> Vec<Vec<usize>> {
        let n = parent.len();
        let mut ancestor_table = vec![vec![usize::MAX; n]; LOG];
        ancestor_table[0] = parent.to_vec();

        for jump in 1..LOG {
            for node in 0..n {
                let half_way = ancestor_table[jump - 1][node];
                if half_way != usize::MAX {
                    ancestor_table[jump][node] = ancestor_table[jump - 1][half_way];
                }
            }
        }
        ancestor_table
    }

    fn build_parent(
        n: usize,
        edges: &[Vec<i32>],
    ) -> (Vec<usize>, Vec<usize>, Vec<usize>, Vec<usize>) {
        let mut neighbors = vec![Vec::new(); n];
        for edge in edges {
            let (left, right) = (edge[0] as usize, edge[1] as usize);
            neighbors[left].push(right);
            neighbors[right].push(left);
        }

        let mut parent = vec![usize::MAX; n];
        let mut depth = vec![0usize; n];
        let mut start = vec![0usize; n];
        let mut end = vec![0usize; n];
        let mut stack = vec![(0usize, false)];
        let mut timer = 0;
        while let Some((curr, is_finished)) = stack.pop() {
            if is_finished {
                end[curr] = timer;
                continue;
            }

            timer += 1;
            start[curr] = timer;
            stack.push((curr, true));
            for &neighbor in neighbors[curr].iter() {
                if neighbor == parent[curr] {
                    continue;
                }
                parent[neighbor] = curr;
                depth[neighbor] = depth[curr] + 1;
                stack.push((neighbor, false));
            }
        }

        (parent, depth, start, end)
    }
}

struct BIT {
    tree: Vec<i32>,
}

impl BIT {
    fn new(chars: &[u8], start_table: &[usize], end_table: &[usize]) -> Self {
        let tree = vec![0; chars.len() + 2];
        let mut obj = Self { tree };

        for (node, (&start, &end)) in start_table.iter().zip(end_table).enumerate() {
            let mask = Self::char_to_i32(chars[node]);
            obj.range_update(start, end, mask);
        }

        obj
    }

    fn update(&mut self, mut pos: usize, mask: i32) {
        while pos < self.tree.len() {
            self.tree[pos] ^= mask;
            pos += Self::low_bit(pos);
        }
    }

    fn query(&self, mut pos: usize) -> i32 {
        let mut result = 0;
        while pos > 0 {
            result ^= self.tree[pos];
            pos -= Self::low_bit(pos);
        }
        result
    }

    fn range_update(&mut self, start: usize, end: usize, mask: i32) {
        self.update(start, mask);
        self.update(end + 1, mask);
    }

    fn char_to_i32(char: u8) -> i32 {
        1 << (char - b'a') as usize
    }

    fn low_bit(pos: usize) -> usize {
        pos & (!pos + 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn to_queries(queries: &[&str]) -> Vec<String> {
        queries.iter().map(|query| query.to_string()).collect()
    }

    #[test]
    fn case1() {
        assert_eq!(
            Solution::palindrome_path(
                3,
                vec![vec![0, 1], vec![1, 2]],
                "aac".to_string(),
                to_queries(&["query 0 2", "update 1 b", "query 0 2"]),
            ),
            vec![true, false]
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::palindrome_path(
                4,
                vec![vec![0, 1], vec![0, 2], vec![0, 3]],
                "abca".to_string(),
                to_queries(&[
                    "query 1 2",
                    "update 0 b",
                    "query 2 3",
                    "update 3 a",
                    "query 1 3"
                ]),
            ),
            vec![false, false, true]
        );
    }

    // 路徑 0-1 是 "ab"，兩個奇數次字元 → false（終點的字元也要算進去）
    #[test]
    fn two_nodes_end_char_counts() {
        assert_eq!(
            Solution::palindrome_path(
                2,
                vec![vec![0, 1]],
                "ab".to_string(),
                to_queries(&["query 0 1"]),
            ),
            vec![false]
        );
    }

    // 起點等於終點：單一字元一定是回文
    #[test]
    fn same_node() {
        assert_eq!(
            Solution::palindrome_path(1, vec![], "z".to_string(), to_queries(&["query 0 0"])),
            vec![true]
        );
    }

    // 5×10^4 個點排成一條鏈：遞迴 DFS 會 stack overflow，迭代 DFS 不會
    #[test]
    fn long_chain() {
        let node_count = 50_000;
        let edges = (1..node_count)
            .map(|node| vec![node - 1, node])
            .collect::<Vec<_>>();
        assert_eq!(
            Solution::palindrome_path(
                node_count,
                edges,
                "a".repeat(node_count as usize),
                // 5 萬個 a → 全偶數；改一個成 b → 49999 個 a + 1 個 b，兩種奇數
                to_queries(&["query 0 49999", "update 25000 b", "query 0 49999"]),
            ),
            vec![true, false]
        );
    }

    // 最笨的方法：沿著 parent 一格一格往上走，數每個字母出現幾次
    fn brute_force_query(parent: &[usize], letters: &[u8], node_a: usize, node_b: usize) -> bool {
        let climb = |start: usize| {
            let mut path = vec![start];
            while parent[*path.last().unwrap()] != usize::MAX {
                path.push(parent[*path.last().unwrap()]);
            }
            path
        };
        let path_a = climb(node_a);
        let path_b = climb(node_b);
        let meeting_point = *path_a.iter().find(|node| path_b.contains(node)).unwrap();

        let mut counts = [0; 26];
        for path in [&path_a, &path_b] {
            for &node in path.iter().take_while(|&&node| node != meeting_point) {
                counts[(letters[node] - b'a') as usize] += 1;
            }
        }
        counts[(letters[meeting_point] - b'a') as usize] += 1;
        counts.iter().filter(|&&count| count % 2 == 1).count() <= 1
    }

    // 隨機產生很多棵小樹和操作，跟笨方法比對答案
    #[test]
    fn matches_brute_force() {
        let mut seed: u64 = 20261029;
        let mut random = |limit: usize| {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            ((seed >> 33) as usize) % limit
        };

        for _round in 0..200 {
            let node_count = 1 + random(12);
            // 節點 node 接到前面隨便一個節點，保證是一棵以 0 為根的樹
            let mut parent = vec![usize::MAX; node_count];
            let mut edges = Vec::new();
            for node in 1..node_count {
                let up = random(node);
                parent[node] = up;
                edges.push(vec![node as i32, up as i32]);
            }
            let mut letters: Vec<u8> = (0..node_count).map(|_| b'a' + random(3) as u8).collect();
            let s = String::from_utf8(letters.clone()).unwrap();

            let mut queries = Vec::new();
            let mut expected = Vec::new();
            for _ in 0..30 {
                if random(2) == 0 {
                    let node = random(node_count);
                    let letter = b'a' + random(3) as u8;
                    letters[node] = letter;
                    queries.push(format!("update {} {}", node, letter as char));
                } else {
                    let node_a = random(node_count);
                    let node_b = random(node_count);
                    queries.push(format!("query {} {}", node_a, node_b));
                    expected.push(brute_force_query(&parent, &letters, node_a, node_b));
                }
            }

            assert_eq!(
                Solution::palindrome_path(node_count as i32, edges, s, queries),
                expected
            );
        }
    }
}
