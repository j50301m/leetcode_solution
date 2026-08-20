struct Solution;

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        let mut c1 = [0; 26];
        let mut c2 = [0; 26];
        for b in word1.bytes() {
            c1[(b - b'a') as usize] += 1;
        }
        for b in word2.bytes() {
            c2[(b - b'a') as usize] += 1;
        }

        for i in 0..26 {
            if (c1[i] == 0) != (c2[i] == 0) {
                return false;
            }
        }

        c1.sort();
        c2.sort();

        c1 == c2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn close(a: &str, b: &str) -> bool {
        Solution::close_strings(a.to_string(), b.to_string())
    }

    #[test]
    fn case_1() {
        // 只用操作 1 交換位置
        assert!(close("abc", "bca"));
    }

    #[test]
    fn case_2() {
        // 長度不同，不可能
        assert!(!close("a", "aa"));
    }

    #[test]
    fn case_3() {
        // c1a2b3 -> a1b2c3，用操作 2 換標籤
        assert!(close("cabbba", "abbccc"));
    }

    #[test]
    fn same_count_multiset_but_different_chars() {
        // u2a1 vs s2x1：次數組合一樣但字元集不同
        assert!(!close("uau", "ssx"));
    }

    #[test]
    fn same_chars_but_different_count_multiset() {
        // {a,b,c,z} 都有，但次數是 [1,2,2,2] vs [1,1,2,3]
        assert!(!close("abbzzca", "babzzcz"));
    }

    #[test]
    fn identical_words() {
        assert!(close("aabbcc", "aabbcc"));
    }

    #[test]
    fn counts_are_permuted() {
        // a3b2 vs b3a2
        assert!(close("aaabb", "bbbaa"));
    }

    #[test]
    fn word1_has_extra_char() {
        assert!(!close("ab", "a"));
    }
}
