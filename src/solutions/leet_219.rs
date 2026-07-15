use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map: HashMap<i32, usize> = HashMap::new(); // key: nums[i], val= index;

        for i in 0..nums.len() {
            if map.contains_key(&nums[i]) {
                let index = map[&nums[i]];
                if ((i - index) as i32).abs() <= k {
                    return true;
                }
            }
            map.insert(nums[i], i);
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let result = Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3);
        assert!(result);
    }

    #[test]
    fn case_2() {
        let result = Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1);
        assert!(result);
    }

    #[test]
    fn case_3() {
        let result = Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2);
        assert!(!result);
    }
}
