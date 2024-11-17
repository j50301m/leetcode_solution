/*
 * @lc app=leetcode id=15 lang=rust
 *
 * [15] 3Sum
 */

// @lc code=start
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort(); // 先排序，這樣可以避免重複並且優化搜索
        let mut result = vec![];
        
        for i in 0..nums.len() {
            // 避免重複的第一個數字
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            
            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                
                match sum.cmp(&0) {
                    std::cmp::Ordering::Equal => {
                        result.push(vec![nums[i], nums[left], nums[right]]);
                        
                        // 避免重複的第二個數字
                        while left < right && nums[left] == nums[left + 1] {
                            left += 1;
                        }
                        // 避免重複的第三個數字
                        while left < right && nums[right] == nums[right - 1] {
                            right -= 1;
                        }
                        
                        left += 1;
                        right -= 1;
                    }
                    std::cmp::Ordering::Less => left += 1,
                    std::cmp::Ordering::Greater => right -= 1,
                }
            }
        }
        
        result
    }
    
}
// @lc code=end

