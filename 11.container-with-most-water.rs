/*
 * @lc app=leetcode id=11 lang=rust
 *
 * [11] Container With Most Water
 */

// @lc code=start
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let mut left = 0;
        let mut right = height.len() - 1;
    
        while right > left {
            let width = (right - left) as i32;
            let left_height = height.get(left).unwrap();
            let right_height = height.get(right).unwrap();
    
            let area = std::cmp::min(left_height, right_height) * width;
            max_area = std::cmp::max(area, max_area);
    
            if right_height > left_height {
                left += 1;
            } else {
                right -= 1;
            }
        }
        max_area
    }
}
// @lc code=end

