/*
 * @lc app=leetcode id=26 lang=rust
 *
 * [26] Remove Duplicates from Sorted Array
 */

// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
    
        let mut write_index = 1;
        for read_index  in 1..nums.len() {
            if nums[read_index] != nums[read_index -1] {
                nums[write_index] = nums[read_index];
                write_index +=1;
            }
        }
    
        write_index as i32
    }
}
// @lc code=end

