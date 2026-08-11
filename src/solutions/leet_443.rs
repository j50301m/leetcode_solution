struct Solution;

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let (mut read, mut write) = (0, 0);
        while read < chars.len() {
            // Read logic
            let c = chars[read];
            let start = read;
            while read < chars.len() && chars[read] == c {
                read += 1;
            }

            // Write logic
            let count = read - start;
            chars[write] = c;
            write += 1;
            if count > 1 {
                for digit in count.to_string().chars() {
                    chars[write] = digit;
                    write += 1;
                }
            }
        }

        chars.truncate(write);
        write as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // 題目要求「原地」壓縮：回傳長度 len，且 chars[..len] 要等於壓縮結果，
    // len 之後的內容不管。所以兩件事都要驗。
    fn check(input: &str, want: &str) {
        let mut chars: Vec<char> = input.chars().collect();
        let len = Solution::compress(&mut chars);
        assert_eq!(len as usize, want.len(), "input={:?} 長度不對", input);
        assert_eq!(
            chars[..len as usize].iter().collect::<String>(),
            want,
            "input={:?} 內容不對",
            input
        );
    }

    #[test]
    fn leetcode_examples() {
        check("aabbccc", "a2b2c3");
        check("a", "a");
        check("aaabbbbbbbbbbbb", "a3b12"); // 12 要拆成 '1','2' 兩格
    }

    #[test]
    fn no_repeats() {
        check("abc", "abc"); // count == 1 不寫數字
        check("ab", "ab");
    }

    #[test]
    fn same_char_in_separate_groups() {
        check("aabbaa", "a2b2a2"); // 同一個字元出現在不同段，不能被合併
        check("abab", "abab");
        check("aabaa", "a2ba2");
    }

    #[test]
    fn all_same() {
        check(&"a".repeat(10), "a10");
        check(&"a".repeat(100), "a100");
        check(&"a".repeat(1000), "a1000"); // 題目上限，數字佔 4 格
    }

    #[test]
    fn boundary_counts() {
        check("aab", "a2b"); // 2 是第一個會寫數字的
        check(&format!("{}b", "a".repeat(9)), "a9b"); // 一位數上限
        check(&format!("{}b", "a".repeat(11)), "a11b"); // 兩位數
    }

    #[test]
    fn case_sensitive() {
        check("aAaA", "aAaA"); // 大小寫是不同字元
        check("aabBB", "a2bB2");
    }

    #[test]
    fn order_is_preserved() {
        check("cccbba", "c3b2a"); // 不是字典序，是出現順序
        check("bbbaaacc", "b3a3c2");
    }
}
