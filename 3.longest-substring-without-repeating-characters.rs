/*
 * @lc app=leetcode id=3 lang=rust
 *
 * [3] Longest Substring Without Repeating Characters
 */

// @lc code=start
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map: std::collections::HashMap<char, usize> = std::collections::HashMap::new();
        let mut max_len = 0;
        let mut start = 0;
        for (end, char) in s.chars().enumerate() {
            if let Some(pre) = map.get(&char) {
                if start <= *pre {
                    start = pre + 1
                }
            }
    
            map.insert(char, end);
    
            max_len = std::cmp::max(end - start + 1, max_len);
        }
    
        max_len as i32
    }
}
// @lc code=end

