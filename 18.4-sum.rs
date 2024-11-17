/*
 * @lc app=leetcode id=18 lang=rust
 *
 * [18] 4Sum
 */

// @lc code=start
impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let target = target as i64;
        nums.sort();
    
        if nums.len() < 4 {
            return result;
        }
    
        for i in 0..nums.len() - 3 {
            // 如果第一層已經排過就跳過
            if i > 0 && nums[i - 1] == nums[i] {
                continue;
            }
    
            for j in i + 1..nums.len() - 2 {
                // 如果第2繩前一個和現在要排的是一樣的數直接跳過
                if j > i + 1 && nums[j - 1] == nums[j] {
                    continue;
                }
    
                let mut left = j + 1;
                let mut right = nums.len() - 1;
    
                while left < right {
                    let sum = nums[i] as i64 + nums[j] as i64 + nums[left] as i64 + nums[right] as i64;
    
                    match sum.cmp(&target) {
                        std::cmp::Ordering::Equal => {
                            result.push(vec![nums[i], nums[j], nums[left], nums[right]]);
    
                            while left < right && nums[left] == nums[left + 1] {
                                left += 1;
                            }
                            while left < right && nums[right] == nums[right - 1] {
                                right -= 1;
                            }
    
                            left += 1;
                            right -= 1;
                        }
                        std::cmp::Ordering::Greater => {
                            right -= 1;
                        }
                        std::cmp::Ordering::Less => {
                            left += 1;
                        }
                    }
                }
            }
        }
        result
    }
}
// @lc code=end

