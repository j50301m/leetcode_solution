struct Solution;

impl Solution {
    pub fn max_operations(mut nums: Vec<i32>, k: i32) -> i32 {
        if nums.len() < 1 {
            return 0;
        }

        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut found = 0;

        nums.sort();
        while left < right {
            let a = nums[left];
            let b = nums[right];
            if a + b == k {
                found += 1;
                left += 1;
                right -= 1;
            } else if a + b > k {
                right -= 1;
            } else {
                left += 1;
            }
        }

        found
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::max_operations(vec![1, 2, 3, 4], 5), 2);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::max_operations(vec![3, 1, 3, 4, 3], 6), 1);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::max_operations(vec![6], 6), 0)
    }
}
