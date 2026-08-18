use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let k = k as usize;
        let set: HashSet<char> = vec!['a', 'e', 'i', 'o', 'u'].into_iter().collect();
        let chars = s.chars().collect::<Vec<_>>();
        let mut curr = 0;
        for i in 0..k {
            if set.contains(&chars[i]) {
                curr += 1;
            }
        }

        let mut best = curr;
        for i in k..chars.len() {
            if set.contains(&chars[i - k]) {
                curr -= 1;
            }
            if set.contains(&chars[i]) {
                curr += 1;
            }
            best = best.max(curr);
        }

        best
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::max_vowels("abciiidef".to_string(), 3), 3);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::max_vowels("aeiou".to_string(), 2), 2);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::max_vowels("tryhard".to_string(), 4), 1);
    }
}
