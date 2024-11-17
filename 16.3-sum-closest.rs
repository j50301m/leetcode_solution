/*
 * @lc app=leetcode id=16 lang=rust
 *
 * [16] 3Sum Closest
 */

// @lc code=start
impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        // Sort first
        nums.sort();

        // Get it a initial value
        let mut closest = nums[0] + nums[1] + nums[2];

        for i in 0..nums.len() - 2 {
            let mut left = i + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];

                if sum == target {
                    return target;
                }

                if (sum-target).abs() < (closest-target).abs() {
                    closest = sum;
                }

                if sum < target {
                    left +=1;
                } else  {
                    right -=1;
                }
            }
        }

        closest
    }
}
// @lc code=end

