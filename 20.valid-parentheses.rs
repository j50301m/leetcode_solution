/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 */

// @lc code=start
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
    
        let mut map = std::collections::HashMap::new();
        map.insert(')', '(');
        map.insert('}', '{');
        map.insert(']', '[');
    
        for char in s.chars() {
            match map.get(&char) {
                Some(char) => {
                    if stack.len() == 0 || stack.last().unwrap() != char {
                        return false;
                    }
    
                    let _ = stack.pop();
                }
                None => {
                    stack.push(char);
                }
            }
        }
    
        stack.len() == 0
    }
}
// @lc code=end

