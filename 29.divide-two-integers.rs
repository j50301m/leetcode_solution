/*
 * @lc app=leetcode id=29 lang=rust
 *
 * [29] Divide Two Integers
 */

// @lc code=start
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }
        if dividend == i32::MIN && divisor == 1 {
            return i32::MIN;
        }
        
        let mut d = (dividend as i64).abs();
        let dv =(divisor as i64).abs();
    
    
        let mut res = 0;
        while d >=dv {
            let mut mul = 1;
            let mut temp =dv;
            while d >= temp <<1 {
                temp = temp <<1;
                mul = mul << 1;
            }
            d -= temp;
            res += mul;
    
        }
    
        if (dividend >0 && divisor < 0) || (dividend < 0 && divisor >0) {
            return -res;
        }
    
        res
    }
}
// @lc code=end

