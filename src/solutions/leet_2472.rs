struct Solution {}

impl Solution {
    pub fn max_palindromes(s: String, k: i32) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut start = 0;
        let mut ans = 0;
        for i in 0..n as i32 {
            for len in [k, k + 1] {
                let j = i as i32 - len + 1;
                if j >= start && Self::is_palindrome(&bytes, j, i) {
                    ans += 1;
                    start = i + 1;
                    break;
                }
            }
        }
        ans
    }

    fn is_palindrome(bytes: &[u8], start: i32, end: i32) -> bool {
        let mut start = start as usize;
        let mut end = end as usize;
        while start < end {
            if bytes[start] != bytes[end] {
                return false;
            }
            start += 1;
            end -= 1;
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn run(s: &str, k: i32) -> i32 {
        Solution::max_palindromes(s.to_string(), k)
    }

    #[test]
    fn case1() {
        assert_eq!(run("abaccdbbd", 3), 2)
    }

    #[test]
    fn case2() {
        assert_eq!(run("adbcda", 2), 0)
    }

    #[test]
    fn k_of_one_takes_every_char() {
        assert_eq!(run("abc", 1), 3)
    }

    #[test]
    fn k_longer_than_string() {
        assert_eq!(run("ab", 3), 0)
    }

    #[test]
    fn whole_string_is_the_only_pick() {
        assert_eq!(run("aaaa", 4), 1)
    }

    #[test]
    fn greedy_splits_into_two() {
        assert_eq!(run("aaaa", 2), 2)
    }

    // Only an odd-length palindrome fits, so the k + 1 window has to fire.
    #[test]
    fn odd_palindrome_needs_the_k_plus_one_window() {
        assert_eq!(run("aba", 2), 1)
    }

    // "abba" at 0..=3 blocks everything after it: "bba" is not a palindrome.
    #[test]
    fn a_taken_span_blocks_later_overlaps() {
        assert_eq!(run("abbabba", 3), 1)
    }

    // Grabbing the long "aabaa" would score 2; taking "aa" "aa" "bb" scores 3.
    #[test]
    fn earliest_end_beats_longest_match() {
        assert_eq!(run("aabaabb", 2), 3)
    }
}
