struct Solution {}

impl Solution {
    pub fn max_num_of_substrings(s: String) -> Vec<String> {
        let bytes = s.as_bytes();
        let mut first = [usize::MAX; 26];
        let mut last = [0usize; 26];

        for i in 0..bytes.len() {
            let pos = (bytes[i] - b'a') as usize;
            if first[pos] == usize::MAX {
                first[pos] = i;
            }
            last[pos] = i;
        }
        let mut candidates = Vec::new();
        for char_pos in 0..26 {
            if first[char_pos] == usize::MAX {
                continue;
            }

            let (l, mut r) = (first[char_pos], last[char_pos]);
            let mut i = l;
            let mut is_representative = true;
            while i <= r {
                let pos = (bytes[i] - b'a') as usize;
                if first[pos] < l {
                    is_representative = false;
                    break;
                }
                if last[pos] > r {
                    r = last[pos];
                }
                i += 1;
            }
            if is_representative {
                candidates.push((l, r));
            }
        }

        candidates.sort_by(|&a, &b| a.1.cmp(&b.1));
        let mut ans = Vec::new();
        let mut curr = 0;
        for (l, r) in candidates {
            if l >= curr {
                ans.push(s[l..=r].to_string());
                curr = r + 1;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &str) -> Vec<String> {
        Solution::max_num_of_substrings(s.to_string())
    }

    #[test]
    fn case1() {
        assert_eq!(run("adefaddaccc"), ["e", "f", "ccc"]);
    }

    #[test]
    fn case2() {
        assert_eq!(run("abbaccd"), ["bb", "cc", "d"]);
    }

    #[test]
    fn single_char() {
        assert_eq!(run("a"), ["a"]);
    }

    #[test]
    fn all_same_char() {
        assert_eq!(run("aaaa"), ["aaaa"]);
    }

    #[test]
    fn all_distinct_chars() {
        // 四個相鄰的單格區間：curr = r + 1 搭配 l >= curr 才收得齊
        assert_eq!(run("abcd"), ["a", "b", "c", "d"]);
    }

    #[test]
    fn fully_interleaved() {
        // a 和 b 互相咬住，只能整串一起拿
        assert_eq!(run("abab"), ["abab"]);
    }

    #[test]
    fn prefers_two_small_over_one_big() {
        // [0,7] "adefadda" 一個人佔掉 "e" 和 "f" 兩格，一換二不划算
        assert_eq!(run("adefaddaccc").len(), 3);
    }

    #[test]
    fn shorter_total_wins_on_tie() {
        // "abba" + "cc" 和 "bb" + "cc" 都是 2 個，總長短的才是答案
        assert_eq!(run("abbacc"), ["bb", "cc"]);
    }

    #[test]
    fn whole_string_is_the_only_interval() {
        assert_eq!(run("cbaccbaa"), ["cbaccbaa"]);
        assert_eq!(run("abcabcbb"), ["abcabcbb"]);
    }

    #[test]
    fn outer_char_excluded() {
        // z 橫跨整串，所以 z 的區間是 [0,5]；abc 各自獨立，贏過它
        assert_eq!(run("zzabcz"), ["a", "b", "c"]);
    }

    #[test]
    fn bailed_char_interval_recovered_by_representative() {
        // "abcac"：c 掃到 index 3 的 a 就淘汰（first[a]=0 < l=2），
        // 它想要的 [0,4] 改由站在 index 0 的 a 算出來，所以候選沒少
        assert_eq!(run("abcac"), ["b"]);
        assert_eq!(run("abcacdd"), ["b", "dd"]);
        // "adefaddaccc"：d 淘汰，[0,7] 由 a 代言
        assert_eq!(run("adefaddaccc"), ["e", "f", "ccc"]);
    }

    #[test]
    fn nested_intervals_pick_the_shortest() {
        // [2,2]"b" ⊂ [1,3]"aba" ⊂ [0,4]"dabad"，最多只取得到 1 個，平手取總長最短
        assert_eq!(run("dabad"), ["b"]);
    }
}
