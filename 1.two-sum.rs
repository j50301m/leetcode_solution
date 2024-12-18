/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() -1 {
            for j in i+1..nums.len() {
                if nums[i] + nums[j] ==target {
                    return vec![i as i32,j as i32];
                }
            }
        }
        return Vec::new();
    }
}

// @lc code=end

