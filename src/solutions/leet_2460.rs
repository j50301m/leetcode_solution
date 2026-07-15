pub struct Solution;

impl Solution {
    pub fn apply_operations(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;

        for i in 0..nums.len() - 1 {
            if nums[i] == nums[i + 1] {
                nums[i] = nums[i] * 2;
                nums[i + 1] = 0;
            }
        }

        let mut k = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[k] = nums[i];
                k += 1;
            }
        }

        for i in k..nums.len() {
            nums[i] = 0;
        }

        nums
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let result = Solution::apply_operations(vec![1, 2, 2, 1, 1, 0]);
        assert_eq!(result, vec![1, 4, 2, 0, 0, 0]);
    }

    #[test]
    fn case_2() {
        let result = Solution::apply_operations(vec![0, 1]);
        assert_eq!(result, vec![1, 0]);
    }
}
