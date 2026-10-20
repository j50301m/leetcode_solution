struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let needle = needle.as_bytes();
        let haystack = haystack.as_bytes();
        let long_borders = Self::build_longest_border(needle);

        let mut border = 0;
        for i in 0..haystack.len() {
            while border > 0 && haystack[i] != needle[border] {
                border = long_borders[border - 1];
            }

            if haystack[i] == needle[border] {
                border += 1;
            }
            if border == needle.len() {
                return (i + 1 - border) as i32;
            }
        }
        -1
    }

    fn build_longest_border(bytes: &[u8]) -> Vec<usize> {
        let mut longest_border = vec![0; bytes.len()];
        let mut border = 0;
        for i in 1..bytes.len() {
            while border > 0 && bytes[i] != bytes[border] {
                border = longest_border[border - 1];
            }

            if bytes[i] == bytes[border] {
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
        assert_eq!(Solution::str_str("sadbutsad".to_string(), "sad".to_string()), 0);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::str_str("leetcode".to_string(), "leeto".to_string()), -1);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::str_str("aaaaab".to_string(), "aab".to_string()), 3);
    }

    #[test]
    fn case4() {
        assert_eq!(Solution::str_str("mississippi".to_string(), "issip".to_string()), 4);
    }

    #[test]
    fn case5() {
        assert_eq!(Solution::str_str("abc".to_string(), "abcd".to_string()), -1);
    }

    #[test]
    fn longest_border_table() {
        assert_eq!(
            Solution::build_longest_border(b"ABACABAB"),
            vec![0, 0, 1, 0, 1, 2, 3, 2]
        );
    }
}
