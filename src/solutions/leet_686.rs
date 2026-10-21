struct Solution {}

impl Solution {
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        let append_cnt = b.len() / a.len() + 1;
        let mut s = a.to_owned();
        for _ in 0..append_cnt {
            s += &a;
        }

        let b = b.as_bytes();
        let s = s.as_bytes();
        let longest_borders = Self::build_longest_border(b);
        let mut border = 0;
        for i in 0..s.len() {
            while border > 0 && s[i] != b[border] {
                border = longest_borders[border - 1];
            }

            if s[i] == b[border] {
                border += 1;
            }

            if border == b.len() {
                // let mut ans = (i + 1) / a.len();
                // if (i + 1) % a.len() != 0 {
                //     ans += 1;
                // }
                // return ans as i32;
                return (i + 1).div_ceil(a.len()) as i32;
            }
        }

        -1
    }

    fn build_longest_border(s: &[u8]) -> Vec<usize> {
        let mut longest_borders = vec![0usize; s.len()];
        let mut border = 0;
        for i in 1..s.len() {
            while border > 0 && s[i] != s[border] {
                border = longest_borders[border - 1];
            }

            if s[border] == s[i] {
                border += 1;
            }

            longest_borders[i] = border;
        }
        longest_borders
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::repeated_string_match("abcd".to_string(), "cdabcdab".to_string()),
            3
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::repeated_string_match("a".to_string(), "aa".to_string()),
            2
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            Solution::repeated_string_match("abc".to_string(), "cabcabca".to_string()),
            4
        );
    }
}
