/*
 * @lc app=leetcode id=6 lang=rust
 *
 * [6] Zigzag Conversion
 */

// @lc code=start
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return  s;
        }
    
        let num_rows = num_rows as usize;
        let chars: Vec<char> = s.chars().collect();
        let mut res:Vec<String> = Vec::with_capacity(num_rows);
        for _ in 0..num_rows {
            res.push(String::new());
        }
    
        let mut idx =0;
        while idx < s.len() {
            for i in 0..num_rows {
                if idx >s.len() - 1 {
                    break;
                }
                let s = res.get_mut(i ).unwrap();
                s.push(chars[idx]);
                idx +=1;
            }
    
            for i in (1..=num_rows-2).rev() {
                if idx >s.len() - 1 {
                    break;
                }
                let s = res.get_mut(i ).unwrap();
                s.push(chars[idx]);
                idx +=1;
            }
        }
    
        res.concat()
    }
}
// @lc code=end

