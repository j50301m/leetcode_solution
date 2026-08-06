struct Solution;

impl Solution {
    pub fn remaining_methods(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let k = k as usize;

        // Build a graph
        let mut graph: Vec<Vec<i32>> = vec![Vec::new(); n];
        for invocation in &invocations {
            let caller = invocation[0];
            let invoker = invocation[1];
            graph[caller as usize].push(invoker);
        }

        // Visit all suspicious node
        let mut suspicious = vec![false; n];
        let mut stack = vec![k];
        suspicious[k] = true;
        while let Some(node) = stack.pop() {
            for &next in &graph[node] {
                let next = next as usize;
                if suspicious[next] {
                    continue;
                }
                suspicious[next] = true;
                stack.push(next);
            }
        }

        for invocation in invocations {
            let outside = invocation[0] as usize;
            let inside = invocation[1] as usize;
            if suspicious[inside] && !suspicious[outside] {
                return (0..n as i32).collect::<Vec<i32>>();
            }
        }

        suspicious
            .iter()
            .enumerate()
            .filter_map(|(i, &s)| (!s).then_some(i as i32))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // 0 從外面呼叫可疑的 1，整組移不掉，回傳全部
    #[test]
    fn case_1() {
        assert_eq!(
            Solution::remaining_methods(4, 1, vec![vec![1, 2], vec![0, 1], vec![3, 2]]),
            vec![0, 1, 2, 3]
        );
    }

    // 可疑組 {0,1,2} 沒有被外部呼叫，移掉後剩 3、4
    #[test]
    fn case_2() {
        assert_eq!(
            Solution::remaining_methods(5, 0, vec![vec![1, 2], vec![0, 2], vec![0, 1], vec![3, 4]]),
            vec![3, 4]
        );
    }

    // 環：2 -> 0 -> 1 -> 2，全部可疑，全部移掉
    #[test]
    fn cycle_removes_all() {
        assert_eq!(
            Solution::remaining_methods(3, 2, vec![vec![1, 2], vec![0, 1], vec![2, 0]]),
            Vec::<i32>::new()
        );
    }

    // 沒有任何呼叫關係，只有 k 自己可疑
    #[test]
    fn no_invocations() {
        assert_eq!(Solution::remaining_methods(3, 1, vec![]), vec![0, 2]);
    }

    // 只有一個方法，就是 k 本身
    #[test]
    fn single_method() {
        assert_eq!(Solution::remaining_methods(1, 0, vec![]), Vec::<i32>::new());
    }

    // 可疑鏈 0 -> 1 -> 2 -> 3，要走滿三層才會標到 3
    #[test]
    fn deep_chain() {
        assert_eq!(
            Solution::remaining_methods(5, 0, vec![vec![0, 1], vec![1, 2], vec![2, 3]]),
            vec![4]
        );
    }

    // 非可疑之間的呼叫 (3 -> 4) 不該擋住移除，只有「非可疑 -> 可疑」才擋
    #[test]
    fn outside_edges_dont_block() {
        assert_eq!(
            Solution::remaining_methods(5, 1, vec![vec![1, 2], vec![3, 4]]),
            vec![0, 3, 4]
        );
    }
}
