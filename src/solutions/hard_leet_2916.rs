struct Solution {}

const MOD: i64 = 1_000_000_007;

struct SegmentTree {
    tree: Vec<i64>,
    lazy: Vec<i64>,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        Self {
            // 使用 4*n 的空間儲存 Segment Tree
            // 2916 的 count 陣列一開始全部都是 0
            tree: vec![0; 4 * n],
            lazy: vec![0; 4 * n],
        }
    }

    fn apply(&mut self, node: usize, start: usize, end: usize, value: i64) {
        // 目前節點管理多少個元素
        let length = (end - start + 1) as i64;

        // 每個元素增加 value
        // 因此區間總和增加 length * value
        self.tree[node] += length * value;

        // 記錄尚未傳給子節點的增量
        self.lazy[node] += value;
    }

    fn push(&mut self, node: usize, start: usize, end: usize) {
        // 沒有待傳遞的更新
        // 或目前已經是葉節點
        if self.lazy[node] == 0 || start == end {
            return;
        }

        let mid = (start + end) / 2;

        let left_child = node * 2;
        let right_child = node * 2 + 1;

        // 目前節點尚未傳下去的增量
        let value = self.lazy[node];

        // 左子節點負責 [start, mid]
        self.apply(left_child, start, mid, value);

        // 右子節點負責 [mid+1, end]
        self.apply(right_child, mid + 1, end, value);

        // 已經傳給子節點，因此清除自己的 lazy
        self.lazy[node] = 0;
    }

    fn range_add(
        &mut self,
        node: usize,
        start: usize,
        end: usize,
        left: usize,
        right: usize,
        value: i64,
    ) {
        if left <= start && end <= right {
            self.apply(node, start, end, value);
            return;
        }
        self.push(node, start, end);

        let mid = (start + end) / 2;
        if left <= mid {
            self.range_add(node * 2, start, mid, left, right, value);
        }

        if right > mid {
            self.range_add(node * 2 + 1, mid + 1, end, left, right, value);
        }
        self.tree[node] = self.tree[node * 2] + self.tree[node * 2 + 1];
    }

    fn query(&mut self, node: usize, start: usize, end: usize, left: usize, right: usize) -> i64 {
        if left <= start && end <= right {
            return self.tree[node];
        }

        self.push(node, start, end);

        let mid = (start + end) / 2;
        let mut result = 0;
        if left <= mid {
            result += self.query(node * 2, start, mid, left, right);
        }
        if right > mid {
            result += self.query(node * 2 + 1, mid + 1, end, left, right);
        }

        result
    }
}

impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut seg = SegmentTree::new(n);
        let mut last_seen = vec![-1_i32; 100_001];
        let mut current = 0_i64;
        let mut answer = 0_i64;

        for right in 0..n {
            let x = nums[right] as usize;
            let left = (last_seen[x] + 1) as usize;
            let old_sum = seg.query(1, 0, n - 1, left, right);

            let length = (right - left + 1) as i64;
            let delta = 2 * old_sum + length;
            current = (current + delta) % MOD;
            seg.range_add(1, 0, n - 1, left, right, 1);
            answer = (answer + current) % MOD;
            last_seen[x] = right as i32;
        }

        answer as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // 暴力解：枚舉所有 subarray，用 HashSet 算 distinct count
    fn brute_force(nums: &[i32]) -> i32 {
        let mut answer = 0_i64;
        for start in 0..nums.len() {
            let mut seen = std::collections::HashSet::new();
            for end in start..nums.len() {
                seen.insert(nums[end]);
                let distinct = seen.len() as i64;
                answer = (answer + distinct * distinct) % MOD;
            }
        }
        answer as i32
    }

    #[test]
    fn case1() {
        assert_eq!(Solution::sum_counts(vec![1, 2, 1]), 15);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::sum_counts(vec![2, 2]), 3);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::sum_counts(vec![5]), 1);
    }

    #[test]
    fn match_brute_force() {
        let cases = vec![
            vec![1, 2, 3, 4],
            vec![1, 1, 1, 1],
            vec![3, 1, 3, 2, 1, 3],
            vec![100000, 1, 100000, 1],
        ];
        for nums in cases {
            assert_eq!(Solution::sum_counts(nums.clone()), brute_force(&nums));
        }
    }
}
