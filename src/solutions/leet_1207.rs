use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut map = HashMap::<i32, i32>::new();
        for &num in arr.iter() {
            *map.entry(num).or_default() += 1;
        }

        let mut set = HashSet::<i32>::new();
        for (_, v) in map.iter() {
            if !set.insert(*v) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        // 1 出現 3 次、2 出現 2 次、3 出現 1 次
        assert!(Solution::unique_occurrences(vec![1, 2, 2, 1, 1, 3]));
    }

    #[test]
    fn case_2() {
        // 1 和 2 都出現 1 次
        assert!(!Solution::unique_occurrences(vec![1, 2]));
    }

    #[test]
    fn case_3() {
        // -3 出現 3 次、0 出現 2 次、1 出現 4 次、10 出現 1 次
        assert!(Solution::unique_occurrences(vec![
            -3, 0, 1, -3, 1, 1, 1, -3, 10, 0
        ]));
    }

    #[test]
    fn single_element() {
        assert!(Solution::unique_occurrences(vec![5]));
    }

    #[test]
    fn all_same_value_is_one_count() {
        assert!(Solution::unique_occurrences(vec![7, 7, 7, 7]));
    }

    #[test]
    fn collision_only_between_last_two() {
        // 1×3, 2×2, 3×1, 4×1  → 3 和 4 撞在一起
        assert!(!Solution::unique_occurrences(vec![1, 1, 1, 2, 2, 3, 4]));
    }
}
