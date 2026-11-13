struct Solution {}

impl Solution {
    pub fn count_non_decreasing_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let k = k as i64;
        let nums: Vec<i64> = nums.into_iter().map(i64::from).collect();

        // 1. Prefix Sum
        let prefix = Self::build_prefix(&nums);

        // 2. Next Greater Element
        let next = Self::next_greater_element(&nums);

        // 3. Binary left
        let (jump, cost) = Self::build_binary_left(&nums, &next, &prefix);

        // 4. Two Pointers
        let mut r = 0usize;
        let mut ans = 0i64;
        for l in 0..n {
            if r < l {
                r = l;
            }

            while r < n {
                let required = Self::query(&nums, &prefix, &jump, &cost, l, r);
                if required > k {
                    break;
                }
                r += 1;
            }
            ans += (r - l) as i64;
        }

        ans
    }

    fn build_prefix(nums: &[i64]) -> Vec<i64> {
        let n = nums.len();
        let mut prefix = vec![0; n + 1];
        for i in 0..n {
            prefix[i + 1] = prefix[i] + nums[i];
        }
        prefix
    }

    fn next_greater_element(nums: &[i64]) -> Vec<usize> {
        let n = nums.len();
        let mut next = vec![n; n];
        let mut stack = Vec::new();
        for i in (0..n).rev() {
            while let Some(&top) = stack.last() {
                if nums[i] < nums[top] {
                    break;
                }
                stack.pop();
            }
            let top = stack.last().unwrap_or(&n);
            next[i] = *top;
            stack.push(i);
        }

        next
    }

    fn build_binary_left(
        nums: &[i64],
        next: &[usize],
        prefix: &[i64],
    ) -> (Vec<Vec<usize>>, Vec<Vec<i64>>) {
        let n = next.len();
        let levels = n.ilog2() as usize + 1;
        let mut jump = vec![vec![n; n + 1]; levels];
        let mut cost = vec![vec![0i64; n + 1]; levels];

        for i in 0..n {
            let end = next[i];
            jump[0][i] = end;
            cost[0][i] = nums[i] * (end - i) as i64 + prefix[i] - prefix[end];
        }

        for p in 1..levels {
            for i in 0..n {
                let mid = jump[p - 1][i];
                jump[p][i] = jump[p - 1][mid];
                cost[p][i] = cost[p - 1][i] + cost[p - 1][mid];
            }
        }

        (jump, cost)
    }

    fn query(
        nums: &[i64],
        prefix: &[i64],
        jump: &[Vec<usize>],
        cost: &[Vec<i64>],
        l: usize,
        r: usize,
    ) -> i64 {
        // query 使用閉區間 [l, r]
        let end = r + 1;
        let mut pos = l;
        let mut total = 0i64;

        // 5. 從最大層往下嘗試跳躍
        for p in (0..jump.len()).rev() {
            if pos == end {
                break;
            }

            let next = jump[p][pos];

            // TODO 9:
            // 如果這次跳躍不超過 end
            // 就累加 Cost 並更新 pos
            if next <= end {
                total += cost[p][pos];
                pos = next;
            }
        }

        // 6. 處理最後不完整區段
        if pos < end {
            // TODO 10:
            // 所有剩餘元素提高到 nums[pos]
            // 使用 Prefix Sum 計算 Cost
            total += nums[pos] * (end - pos) as i64 - prefix[end] + prefix[pos];
        }

        total
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::count_non_decreasing_subarrays(vec![6, 3, 1, 2, 4, 4], 7),
            17
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::count_non_decreasing_subarrays(vec![6, 3, 1, 3, 6], 4),
            12
        );
    }

    #[test]
    fn max_not_at_left() {
        // [1, 6] 已經是非遞減，3 個子陣列都合法
        assert_eq!(Solution::count_non_decreasing_subarrays(vec![1, 6], 0), 3);
    }
}
