/*
 * @lc app=leetcode id=5 lang=rust
 *
 * [5] Longest Palindromic Substring
 */

// @lc code=start
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars: Vec<char> =s.chars().collect();
        let mut left = 0;
        let mut right = 0;
    
        fn expend(s: &Vec<char>,mut left: i32,mut right:i32, max_left:&mut i32,max_right: &mut i32) {
            while left >=0 && right < s.len() as i32 && s[left as usize] == s[right as usize] {
                if right-left  > *max_right- *max_left {
                    *max_left = left;
                    *max_right = right;
                }
                left-=1;
                right+=1;
            }
        }
    
        for i in 0..chars.len() {
            expend(&chars, i as i32,i as i32, &mut left, &mut right);
            expend(&chars, i as i32,(i+1) as i32,&mut left, &mut right);
        }
    
        s[left as usize..(right+1) as usize].to_string()
    }
}
// @lc code=end

