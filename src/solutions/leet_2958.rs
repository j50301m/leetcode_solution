use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut left = 0;
        let mut best = 0;
        let mut map: HashMap<i32, i32> = HashMap::new();
        for right in 0..n {
            if let Some(val) = map.get_mut(&nums[right]) {
                *val += 1;
            } else {
                map.insert(nums[right], 1);
            }

            while map[&nums[right]] > k {
                *map.get_mut(&nums[left]).unwrap() -= 1;
                left += 1;
            }

            best = best.max(right + 1 - left);
        }

        best as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::max_subarray_length(vec![1, 2, 3, 1, 2, 3, 1, 2], 2),
            6
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::max_subarray_length(vec![1, 2, 1, 2, 1, 2, 1, 2], 1),
            2
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::max_subarray_length(vec![5, 5, 5, 5, 5, 5, 5], 4),
            4
        );
    }

    #[test]
    fn case_4() {
        assert_eq!(
            Solution::max_subarray_length(vec![1, 2, 3, 1, 2, 5, 1, 2], 2),
            6
        );
    }

    #[test]
    fn case_5() {
        assert_eq!(Solution::max_subarray_length(vec![1], 1), 1);
    }

    #[test]
    fn case_6() {
        assert_eq!(Solution::max_subarray_length(vec![1, 1, 1, 1], 10), 4);
    }
}
