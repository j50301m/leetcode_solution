struct Solution {}

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        Self::approach1(s)
        // Self::approach2(s)
    }

    // Use divisor trait
    fn approach1(s: String) -> bool {
        if s.len() == 1 {
            return false;
        }
        let n = s.len();
        let str_slice = s.as_str();
        for i in 1..=n / 2 {
            if n % i == 0 {
                let portion = &str_slice[..i];
                if portion.to_owned() + str_slice == str_slice.to_owned() + portion {
                    return true;
                }
            }
        }

        false
    }

    // Using KMP algorithm
    fn approach2(s: String) -> bool {
        if s.len() == 1 {
            return false;
        }

        let mut longest_borders = vec![0usize; s.len()];
        let mut border = 0usize;
        let bytes = s.as_bytes();
        for i in 1..s.len() {
            while border > 0 && bytes[border] != bytes[i] {
                border = longest_borders[border - 1];
            }

            if bytes[border] == bytes[i] {
                border += 1;
            }

            longest_borders[i] = border;
        }

        let last = longest_borders[longest_borders.len() - 1];

        last >= (s.len() - last) && last % (s.len() - last) == 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::repeated_substring_pattern("abab".to_string()),
            true
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::repeated_substring_pattern("aabaaba".to_string()),
            false
        );
    }
}
