struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s = s.as_bytes();
        let t = t.as_bytes();

        if s.len() == 0 {
            return true;
        }

        if s.len() == 0 || t.len() == 0 {
            return false;
        }

        let mut ptr = 0;
        for &byte in t {
            if ptr == s.len() {
                return true;
            } else if byte == s[ptr] {
                ptr += 1;
            }
        }

        ptr == s.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::is_subsequence("abc".to_string(), "ahbgdc".to_string()),
            true
        )
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::is_subsequence("axc".to_string(), "ahbgdc".to_string()),
            false
        )
    }
}
