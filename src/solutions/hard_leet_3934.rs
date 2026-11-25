use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn smallest_unique_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let base = 10u64.max(*nums.iter().max().unwrap() as u64 + 1);

        let mut left = 1;
        let mut right = n;
        while left < right {
            let mid = left + (right - left) / 2;
            if Self::has_unique(&nums, base, mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left as i32
    }

    fn has_unique(nums: &[i32], base: u64, mid: usize) -> bool {
        let first = Self::window_key(nums, base, mid, 1000000007);
        let second = Self::window_key(nums, base, mid, 100000009);

        let mut map: HashMap<_, _> = HashMap::new();
        for key in first.into_iter().zip(second) {
            *map.entry(key).or_insert(0) += 1;
        }

        map.into_iter().find(|&(_, val)| val == 1).is_some()
    }

    fn window_key(nums: &[i32], base: u64, win_len: usize, m: u64) -> Vec<u64> {
        let mut weight = 1;
        for _ in 0..win_len - 1 {
            weight = weight * base % m;
        }

        let mut keys = Vec::new();
        let mut key = 0;
        for i in 0..win_len {
            key = (key * base + nums[i] as u64) % m;
        }
        keys.push(key);

        for i in win_len..nums.len() {
            let previous = nums[i - win_len] as u64;
            let next = nums[i] as u64;

            let removed = previous * weight % m;
            let rest = (key + m - removed) % m;
            key = (rest * base + next) % m;
            keys.push(key);
        }

        keys
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::smallest_unique_subarray(vec![3, 3, 3]), 3);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::smallest_unique_subarray(vec![2, 1, 2, 3, 3]), 1);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::smallest_unique_subarray(vec![1, 2, 2, 1]), 2);
    }

    #[test]
    fn case4() {
        assert_eq!(
            Solution::smallest_unique_subarray(vec![3036, 93229, 3036, 93229, 3036]),
            3
        );
    }
}
