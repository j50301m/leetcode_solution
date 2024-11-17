/*
 * @lc app=leetcode id=14 lang=rust
 *
 * [14] Longest Common Prefix
 */

// @lc code=start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = "";
        for (i,s) in strs.iter().enumerate() {
            if i == 0 {
                prefix = &s;
                continue;
            }
    
            let count = s
                .chars()
                .zip(prefix.chars())
                .take_while(|(a, b)| a == b)
                .count();
    
            if count == 0 {
                return "".to_string();
            } else {
                prefix = &s[0..count]
            }
        }
    
        prefix.to_string()
    }
}
// @lc code=end

