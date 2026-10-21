struct Solution {}

impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        // let n = s.len();
        let bytes = s.as_bytes();
        let mut rev_bytes = bytes.to_vec();
        rev_bytes.reverse();

        let mut max_border = 0;
        let mut border = 0;
        let longest_border = Self::build_longest_border(bytes);
        for i in 0..rev_bytes.len() {
            while border > 0 && rev_bytes[i] != bytes[border] {
                border = longest_border[border - 1];
            }

            if rev_bytes[i] == bytes[border] {
                border += 1;
            }

            max_border = border.max(border);
        }

        let result: Vec<u8> = bytes[max_border..]
            .iter()
            .rev()
            .chain(bytes.iter())
            .copied()
            .collect();

        String::from_utf8(result).unwrap()
    }

    fn build_longest_border(s: &[u8]) -> Vec<usize> {
        let mut longest_border = vec![0usize; s.len()];
        let mut border = 0;
        for i in 1..s.len() {
            while border > 0 && s[i] != s[border] {
                border = longest_border[border - 1];
            }

            if s[i] == s[border] {
                border += 1;
            }

            longest_border[i] = border;
        }
        longest_border
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::shortest_palindrome("aacecaaa".to_string()),
            "aaacecaaa".to_string()
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::shortest_palindrome("abcd".to_string()),
            "dcbabcd".to_string()
        );
    }
}
