struct Solution;

impl Solution {
    pub fn valid_sequence(word1: String, word2: String) -> Vec<i32> {
        let n = word1.len();
        let m = word2.len();
        let char1 = word1.chars().collect::<Vec<_>>();
        let char2 = word2.chars().collect::<Vec<_>>();
        let mut dp = vec![0; n + 1];

        for i in (0..n).rev() {
            let k = dp[i + 1]; // How many word can be match
            if k == m || char1[i] != char2[m - 1 - k] {
                dp[i] = k;
                continue;
            }
            dp[i] = k + 1;
        }

        let mut result: Vec<i32> = Vec::new();
        let mut used = false;
        for i in 0..n {
            let pos: usize = result.len();

            if pos == m {
                break;
            }
            if char1[i] != char2[pos] {
                let remaining = m - 1 - pos;
                if !used && dp[i + 1] >= remaining {
                    used = true;
                    result.push(i as i32);
                }
            } else {
                result.push(i as i32);
            }
        }

        if result.len() == m {
            return result;
        }
        vec![]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn run(w1: &str, w2: &str) -> Vec<i32> {
        Solution::valid_sequence(w1.to_string(), w2.to_string())
    }

    // LeetCode 範例：容錯用在 index 0
    #[test]
    fn case_1() {
        assert_eq!(run("vbcca", "abc"), vec![0, 1, 2]);
    }

    // LeetCode 範例：index 0 吞不起錯（後路不夠），容錯延到 index 2
    #[test]
    fn case_2() {
        assert_eq!(run("bacdc", "abc"), vec![1, 2, 4]);
    }

    // LeetCode 範例：要改 2 個字才行 → 無解
    #[test]
    fn case_3() {
        assert_eq!(run("aaaaaa", "aaabc"), vec![]);
    }

    // LeetCode 範例：完全不需要容錯
    #[test]
    fn case_4() {
        assert_eq!(run("abc", "ab"), vec![0, 1]);
    }

    // 容錯只有一次：貪心一路吃下去會在 index 3 偷用第二次錯
    #[test]
    fn only_one_mismatch_allowed() {
        assert_eq!(run("caacb", "cabb"), vec![0, 1, 2, 4]);
    }

    // 同上，最小反例：拿 [0,1] 會錯兩個字，正解要跳過 index 1
    #[test]
    fn only_one_mismatch_allowed_small() {
        assert_eq!(run("ababb", "ca"), vec![0, 2]);
    }

    // result 填滿後還在跑迴圈 → char2[pos] 越界
    #[test]
    fn stops_after_filling() {
        assert_eq!(run("zzz", "z"), vec![0]);
        assert_eq!(run("abcd", "abd"), vec![0, 1, 2]);
    }

    // word1 比 word2 短，湊不出長度 → 無解
    #[test]
    fn word1_too_short() {
        assert_eq!(run("ab", "abc"), vec![]);
    }

    // 單字元邊界：dp 全 0、容錯用在唯一一格
    #[test]
    fn single_char() {
        assert_eq!(run("a", "a"), vec![0]);
        assert_eq!(run("a", "b"), vec![0]);
    }

    // 容錯用在 index 0，後面全靠精準配對補回來
    #[test]
    fn mismatch_at_head() {
        assert_eq!(run("xbzc", "abc"), vec![0, 1, 3]);
    }
}
