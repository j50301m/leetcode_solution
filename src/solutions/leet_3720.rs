struct Solution;

impl Solution {
    pub fn lex_greater_permutation_old(s: String, target: String) -> String {
        let t = target.as_bytes();
        let mut cnt = [0usize; 26];
        for &b in s.as_bytes() {
            cnt[(b - b'a') as usize] += 1;
        }

        // 1. 先盡量貼著 target 走，看最多能貼合幾位；cnt 剩下的就是「用掉前綴後還有的字元」
        let mut matched = 0;
        while matched < t.len() && cnt[(t[matched] - b'a') as usize] > 0 {
            cnt[(t[matched] - b'a') as usize] -= 1;
            matched += 1;
        }

        // 2. 從最深的位置往回找第一個「放得下 > target[i] 的字元」的位置
        for i in (0..=matched.min(t.len() - 1)).rev() {
            if i < matched {
                cnt[(t[i] - b'a') as usize] += 1; // 這位本來貼合 target，先還回去
            }
            let idx = (t[i] - b'a') as usize;
            if let Some(c) = (idx + 1..26).find(|&c| cnt[c] > 0) {
                cnt[c] -= 1;
                // 3. 前綴照抄 target，這位放 c，剩下的由小到大接上
                let mut out = t[..i].to_vec();
                out.push(c as u8 + b'a');
                for (j, &n) in cnt.iter().enumerate() {
                    out.extend(std::iter::repeat(j as u8 + b'a').take(n));
                }
                return String::from_utf8(out).unwrap();
            }
        }

        String::new()
    }

    pub fn lex_greater_permutation(s: String, target: String) -> String {
        let t = target.as_bytes();
        let mut cnt = vec![0usize; 26];
        for &b in s.as_bytes() {
            cnt[(b - b'a') as usize] += 1;
        }

        let mut matched = 0;
        while matched < t.len() && cnt[(t[matched] - b'a') as usize] > 0 {
            cnt[(t[matched] - b'a') as usize] -= 1;
            matched += 1;
        }

        for i in (0..=matched.min(t.len() - 1)).rev() {
            let idx = (t[i] - b'a') as usize;
            if i < matched {
                cnt[idx] += 1;
            }

            if let Some(c) = (idx + 1..26).find(|j| cnt[*j] > 0) {
                cnt[c] -= 1;

                let mut ans = t[0..i].to_vec();
                ans.push(c as u8 + b'a');
                for (j, &val) in cnt.iter().enumerate() {
                    ans.extend(std::iter::repeat(j as u8 + b'a').take(val));
                }

                return String::from_utf8(ans).unwrap();
            }
        }

        String::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn run(s: &str, target: &str) -> String {
        Solution::lex_greater_permutation(s.to_string(), target.to_string())
    }

    // 直接照 target 排出來會等於 target，但換一種排法還是有更大的
    #[test]
    fn case1() {
        assert_eq!(run("ba", "ab"), "ba");
    }

    // 前面已經比 target 大了，後面要直接接「剩下字元由小到大」
    #[test]
    fn case2() {
        assert_eq!(run("abcd", "aadc"), "abcd");
    }

    // 完全排不出比 target 大的
    #[test]
    fn case3() {
        assert_eq!(run("ba", "zz"), "");
    }

    // 需要回頭在前一位改用更大的字元
    #[test]
    fn case4() {
        assert_eq!(run("abc", "acb"), "bac");
    }

    // 第 1 位卡住，要退回第 0 位改成 'c'
    #[test]
    fn case5() {
        assert_eq!(run("cab", "bzz"), "cab");
    }

    #[test]
    fn case6() {
        assert_eq!(run("ab", "aa"), "ab");
    }

    // 只有一種排法且等於 target
    #[test]
    fn case7() {
        assert_eq!(run("aaa", "aaa"), "");
    }
}
