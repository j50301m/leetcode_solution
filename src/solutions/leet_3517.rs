use std::format;

struct Solution;

impl Solution {
    pub fn smallest_palindrome(s: String) -> String {
        let mut words = s.bytes().collect::<Vec<_>>();

        let half = words.len() / 2;

        let (mid, prefix) = if words.len() % 2 == 0 {
            (String::from(""), &mut words[0..half])
        } else {
            let mid = words[half] as char;
            (mid.to_string(), &mut words[0..half])
        };

        prefix.sort();
        let prefix = prefix.to_vec();
        let subfix = prefix.iter().rev().copied().collect::<Vec<_>>();
        let s1 = String::from_utf8(prefix).unwrap();
        let s2 = String::from_utf8(subfix).unwrap();

        format!("{}{}{}", s1, mid, s2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let s = "babab".to_string();
        let result = Solution::smallest_palindrome(s);
        assert_eq!(result, "abbba".to_string());
    }
}
