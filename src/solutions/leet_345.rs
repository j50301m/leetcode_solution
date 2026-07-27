use std::vec;

struct Solution;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let vowels = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        let mut bytes = s.into_bytes();
        let mut right = bytes.len() - 1;
        let mut left = 0;

        while left < right {
            let l = bytes[left] as char;
            let r = bytes[right] as char;
            let mut is_found = true;

            if !vowels.contains(&l) {
                left += 1;
                is_found = false;
            }
            if !vowels.contains(&r) {
                right -= 1;
                is_found = false;
            }

            if is_found {
                (bytes[left], bytes[right]) = (bytes[right], bytes[left]);
                left += 1;
                right -= 1;
            }
        }

        String::from_utf8(bytes).unwrap()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn case_1() {
        let s = String::from("IceCreAm");
        let result = Solution::reverse_vowels(s);
        assert_eq!(result, "AceCreIm");
    }
}
