/*
 * @lc app=leetcode id=28 lang=rust
 *
 * [28] Find the Index of the First Occurrence in a String
 */

// @lc code=start
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let s: Vec<char> = haystack.chars().collect();
        let target:Vec<char> = needle.chars().collect();
    
        if s.len() < target.len() {
            return -1
        }
    
        for i in 0..s.len() {
            let mut curr =i;
            let mut is_found =true;
            for j in 0..target.len() {
                if curr >= s.len() || s[curr]!=target[j]{
                    is_found =false;
                    break;
                }
                curr +=1;
            }
            
            if is_found {
                return i as i32;
            }
        }
        
        -1
    }
}
// @lc code=end

