pub struct Solution;

impl Solution {
    pub fn move_zeros(nums: &mut Vec<i32>) {
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
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::move_zeros(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }

    #[test]
    fn case_2() {
        let mut nums = vec![0];
        Solution::move_zeros(&mut nums);
        assert_eq!(nums, vec![0]);
    }
}
