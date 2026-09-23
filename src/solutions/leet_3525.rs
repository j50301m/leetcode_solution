struct Solution {}

struct SegmentTree {
    tree: Vec<(i32, Vec<i32>)>,
    nums_len: usize,
    k: i32,
}
impl SegmentTree {
    fn new(nums: &[i32], k: i32) -> SegmentTree {
        let n = nums.len();
        let mut tree = vec![(0, vec![0; k as usize]); n.next_power_of_two() * 2];
        Self::build(1, 0, n - 1, nums, k, &mut tree);
        Self {
            tree,
            nums_len: n,
            k,
        }
    }

    fn build(
        node: usize,
        left: usize,
        right: usize,
        nums: &[i32],
        k: i32,
        tree: &mut Vec<(i32, Vec<i32>)>,
    ) {
        if left == right {
            let prod = nums[left];
            tree[node].0 = prod % k;
            tree[node].1[(prod % k) as usize] = 1;
            return;
        }

        let mid = left + (right - left) / 2;
        let left_child = 2 * node;
        let right_child = 2 * node + 1;
        Self::build(left_child, left, mid, nums, k, tree);
        Self::build(right_child, mid + 1, right, nums, k, tree);

        tree[node] = Self::merge(&tree[left_child], &tree[right_child], k);
    }

    fn merge(l: &(i32, Vec<i32>), r: &(i32, Vec<i32>), k: i32) -> (i32, Vec<i32>) {
        let l_prod = l.0;
        let r_prod = r.0;
        let l_cnt = &l.1;
        let r_cnt = &r.1;

        let mut cnt = l_cnt.clone();
        for i in 0..k {
            let new_r = (l_prod * i) % k;
            cnt[new_r as usize] += r_cnt[i as usize];
        }

        ((l_prod * r_prod) % k, cnt)
    }

    fn update(&mut self, query: Vec<i32>) {
        Self::modify(
            1,
            0,
            self.nums_len - 1,
            query[0] as usize,
            query[1],
            self.k,
            &mut self.tree,
        );
    }

    fn modify(
        node: usize,
        left: usize,
        right: usize,
        index: usize,
        val: i32,
        k: i32,
        tree: &mut Vec<(i32, Vec<i32>)>,
    ) {
        if left == right && left == index {
            let mut cnt = vec![0; k as usize];
            let prod = val % k;
            cnt[prod as usize] = 1;
            tree[node].0 = prod;
            tree[node].1 = cnt;
            return;
        }

        let mid = left + (right - left) / 2;
        let left_child = 2 * node;
        let right_child = 2 * node + 1;
        if index <= mid {
            Self::modify(left_child, left, mid, index, val, k, tree);
        } else {
            Self::modify(right_child, mid + 1, right, index, val, k, tree);
        }

        tree[node] = Self::merge(&tree[left_child], &tree[right_child], k);
    }

    fn find_range(&self, start: usize, remainder: i32) -> i32 {
        let (_, cnt) = self.query(1, 0, self.nums_len - 1, start, self.nums_len - 1);
        cnt[remainder as usize]
    }

    fn query(
        &self,
        node: usize,
        left: usize,
        right: usize,
        left_target: usize,
        right_target: usize,
    ) -> (i32, Vec<i32>) {
        if left_target <= left && right <= right_target {
            return self.tree[node].clone();
        }

        let mid = left + (right - left) / 2;
        let mut acc = (1, vec![0; self.k as usize]);

        if left_target <= mid {
            let l = self.query(2 * node, left, mid, left_target, right_target);
            acc = Self::merge(&acc, &l, self.k);
        }
        if right_target > mid {
            let r = self.query(2 * node + 1, mid + 1, right, left_target, right_target);
            acc = Self::merge(&acc, &r, self.k);
        }

        acc
    }
}

impl Solution {
    pub fn result_array(nums: Vec<i32>, k: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut tree = SegmentTree::new(&nums, k);
        queries
            .into_iter()
            .map(|q| {
                let (start, x) = (q[2] as usize, q[3]);
                tree.update(q);
                tree.find_range(start, x)
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::result_array(
                vec![1, 2, 3, 4, 5],
                3,
                vec![vec![2, 2, 0, 2], vec![3, 3, 3, 0], vec![0, 1, 0, 1]]
            ),
            vec![2, 2, 2]
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::result_array(
                vec![1, 2, 4, 8, 16, 32],
                4,
                vec![vec![0, 2, 0, 2], vec![0, 2, 0, 1]]
            ),
            vec![1, 0]
        );
    }

    #[test]
    fn single_element() {
        assert_eq!(
            Solution::result_array(vec![7], 1, vec![vec![0, 9, 0, 0]]),
            vec![1]
        );
    }
}
