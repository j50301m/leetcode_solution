/*
 * @lc app=leetcode id=17 lang=rust
 *
 * [17] Letter Combinations of a Phone Number
 */

// @lc code=start
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut result = Vec::new();
        let digits: Vec<char> = digits.chars().collect();
    
        if digits.len() == 0 {
            return result;
        }
    
        let mut map = std::collections::HashMap::new();
        map.insert('2', "abc");
        map.insert('3', "def");
        map.insert('4', "ghi");
        map.insert('5', "kjl");
        map.insert('6', "mno");
        map.insert('7', "pqrs");
        map.insert('8', "tuv");
        map.insert('9', "wxyz");
    
        fn backtrace(
            map: &std::collections::HashMap<char, &str>,
            digits: &Vec<char>,
            now_index: usize,
            combo: String,
            result: &mut Vec<String>,
        ) {
            if now_index == digits.len() {
                result.push(combo);
                return;
            }
    
            let digit = digits.get(now_index).unwrap();
            let chars = *map.get(digit).unwrap_or(&" ");
    
            for char in chars.chars() {
                let now_combo = format!("{}{}", combo, char);
                backtrace(map, digits, now_index + 1, now_combo, result);
            }
        }
    
        backtrace(&map, &digits, 0, String::new(), &mut result);
    
        result
    }
}
// @lc code=end

