struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut used = 0;
        let mut left = 0;
        let mut best = 0;
        for right in 0..n {
            let curr = nums[right];
            if curr == 0 {
                used += 1
            }
            while used > 1 {
                if nums[left] == 0 {
                    used -= 1;
                }
                left += 1;
            }

            best = best.max(right - left);
        }

        best as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::longest_subarray(vec![1, 1, 0, 1]), 3);
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::longest_subarray(vec![0, 1, 1, 1, 0, 1, 1, 0, 1]),
            5
        );
    }
}
