/*
 * @lc app=leetcode id=4 lang=rust
 *
 * [4] Median of Two Sorted Arrays
 */

// @lc code=start
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if nums2.len() <nums1.len() {
            return Self::find_median_sorted_arrays(nums2, nums1);
        }
    
        let total_len = nums1.len() +nums2.len();
        let mut start = 0;
        let mut end = nums1.len();
    
        while start <= end {
            let cut1 = (end + start) / 2;
            let cut2 =(total_len / 2) - cut1;
    
            let num1_left =  if cut1 == 0 {i32::MIN} else {nums1[cut1-1]};
            let num1_right = if cut1 == nums1.len() {i32::MAX} else {nums1[cut1]};
            let num2_left = if cut2 == 0 {i32::MIN} else {nums2[cut2-1]};
            let num2_right = if cut2 == nums2.len() {i32::MAX} else {nums2[cut2]};
    
            if num1_left> num2_right {
                end = cut1 - 1;
            }else if num1_right < num2_left {
                start = cut1 + 1;
            } else {
                if total_len %2 == 0 {
                    let max = std::cmp::max(num1_left,num2_left);
                    let min = std::cmp::min(num1_right,num2_right);
                    return (max + min) as f64 / 2.0;
                } else {
                    return std::cmp::min(num1_right,num2_right) as f64;
                }
            }
        }

        0.0
    }
}
// @lc code=end

