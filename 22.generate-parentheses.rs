/*
 * @lc app=leetcode id=22 lang=rust
 *
 * [22] Generate Parentheses
 */

// @lc code=start
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
    
        fn backtrace(result: &mut Vec<String>,n: i32,now_open: i32,now_close:i32,curr: String) {
            if now_open == n &&  now_close == n {
                result.push(curr.clone());
                return;
            }
    
            if now_open < n {
                let mut next = curr.clone();
                next.push('(');
                backtrace(result, n , now_open+1 , now_close, next);
            }
    
            if  now_close < now_open {
                let mut next = curr.clone();
                next.push(')');
                backtrace(result,n, now_open, now_close +1,next);
            }
        }
    
        let mut result = Vec::new();
        backtrace(result.as_mut(),n,0,0,String::new());
    
        result
    }
}
// @lc code=end

