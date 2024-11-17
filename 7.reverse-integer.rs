/*
 * @lc app=leetcode id=7 lang=rust
 *
 * [7] Reverse Integer
 */

// @lc code=start
impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let is_negative;
    
        match x.cmp(&0) {
            std::cmp::Ordering::Greater => is_negative = false,
            std::cmp::Ordering::Less => is_negative = true,
            std::cmp::Ordering::Equal => return 0,
        }
    
        x = x.abs();
        let mut result: i32 = 0;
        while x > 0 {
            match result.checked_mul(10).and_then(|r| r.checked_add(x % 10)) {
                Some(val) => result = val,
                None => return 0,
            }
    
            x = x / 10;
        }
    
        if is_negative {
            return -result;
        }
        result
    }
    
}
// @lc code=end

