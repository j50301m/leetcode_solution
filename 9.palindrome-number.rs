/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false
        }
        let mut tmp = x;
        let mut s: i32 = 0;
        while tmp != 0{
            s = s*10+tmp%10;
            tmp /=10;
        }
        x == s
    }
}
// @lc code=end

