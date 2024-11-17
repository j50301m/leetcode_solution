/*
 * @lc app=leetcode id=13 lang=rust
 *
 * [13] Roman to Integer
 */

// @lc code=start
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        let mut pre = 0;
    
        for c in s.chars() {
            let current = to_int(&c.to_string());
            if current > pre {
                result += current - 2 * pre;
            } else {
                result += current;
            }
            pre = current;
        }
    
        println!("{}", result);
        result
    }
}

fn to_int(s: &str) -> i32 {
    match s {
        "I" => 1,
        "V" => 5,
        "X" => 10,
        "L" => 50,
        "C" => 100,
        "D" => 500,
        "M" => 1000,
        _ => 0,
    }
}
// @lc code=end

