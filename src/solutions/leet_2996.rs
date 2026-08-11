use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn missing_integer(nums: Vec<i32>) -> i32 {
        let mut sum = nums[0];
        let mut i = 1;
        while i < nums.len() {
            if nums[i] != nums[i - 1] + 1 {
                break;
            }
            sum += nums[i];
            i += 1;
        }
        let set: HashSet<i32> = nums.iter().copied().collect();
        while set.contains(&sum) {
            sum += 1;
        }

        sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::missing_integer(vec![1, 2, 3, 2, 5]), 6);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::missing_integer(vec![3, 4, 5, 1, 12, 14, 13]), 15);
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::missing_integer(vec![4, 5, 6, 7, 8, 8, 9, 4, 3, 2, 7]),
            30
        );
    }

    #[test]
    fn case_4() {
        assert_eq!(Solution::missing_integer(vec![38]), 39);
    }
}
