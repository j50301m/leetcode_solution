use std::collections::BTreeMap;

struct Solution;

impl Solution {
    pub fn lex_palindromic_permutation(s: String, target: String) -> String {
        let target_chars: Vec<char> = target.chars().collect();
        let mut map: BTreeMap<char, i32> = BTreeMap::new();
        for c in s.chars() {
            *map.entry(c).or_insert(0) += 1;
        }

        let mut tolerant = if s.len() % 2 == 0 { 0 } else { 1 };
        for (_k, val) in map.iter() {
            if val % 2 == 1 {
                tolerant -= 1;
            }
            if tolerant < 0 {
                return String::new();
            }
        }

        let mut matched = 0usize;
        for i in 0..target.len() / 2 {
            let Some(val) = map.get_mut(&target_chars[i]) else {
                break;
            };
            if *val < 2 {
                break;
            }

            *val -= 2;
            matched += 1;
        }

        if matched == target.len() / 2 {
            let mut candidate = target_chars[..matched].to_vec();
            if let Some((mid, _)) = map.iter().find(|&(_k, v)| *v == 1) {
                candidate.push(*mid);
            }
            candidate.extend(target_chars[..matched].iter().rev());
            if candidate > target_chars {
                return candidate.iter().collect();
            }
        }

        for i in (0..=matched).rev() {
            if i < matched {
                map.entry(target_chars[i]).and_modify(|x| *x += 2);
            }

            let Some((found_c, found_cnt)) = map
                .iter_mut()
                .find(|&(c, ref cnt)| *c > target_chars[i] && **cnt > 1)
            else {
                continue;
            };

            let mut result = target_chars[..i].to_vec();
            result.push(*found_c);
            *found_cnt -= 2;

            for (k, cnt) in map.iter_mut() {
                while *cnt > 1 {
                    result.push(*k);
                    *cnt -= 2;
                }
            }

            let mut rev_result = result.clone().into_iter().rev().collect();
            let Some(remaining) = map.iter().find(|&(_k, v)| *v == 1) else {
                result.append(&mut rev_result);
                return result.iter().collect();
            };
            result.push(*remaining.0);
            result.append(&mut rev_result);
            return result.iter().collect();
        }

        String::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn run(s: &str, target: &str) -> String {
        Solution::lex_palindromic_permutation(s.to_string(), target.to_string())
    }

    // ponytail: 直接窮舉所有排列當 oracle，只夠小 n 用（n <= 6）
    fn brute(s: &str, target: &str) -> String {
        fn dfs(
            cnt: &mut [usize; 26],
            cur: &mut Vec<u8>,
            n: usize,
            target: &[u8],
        ) -> Option<String> {
            if cur.len() == n {
                let is_pal = cur.iter().eq(cur.iter().rev());
                return (is_pal && cur.as_slice() > target)
                    .then(|| String::from_utf8(cur.clone()).unwrap());
            }
            for c in 0..26 {
                if cnt[c] == 0 {
                    continue;
                }
                cnt[c] -= 1;
                cur.push(c as u8 + b'a');
                if let Some(r) = dfs(cnt, cur, n, target) {
                    return Some(r);
                }
                cur.pop();
                cnt[c] += 1;
            }
            None
        }

        let mut cnt = [0usize; 26];
        for b in s.bytes() {
            cnt[(b - b'a') as usize] += 1;
        }
        dfs(&mut cnt, &mut Vec::new(), s.len(), target.as_bytes()).unwrap_or_default()
    }

    // 題目範例 1：回文排列只有 "abba" / "baab"，比 target 大的最小者是 "baab"
    #[test]
    fn case1() {
        assert_eq!(run("baba", "abba"), "baab");
    }

    // 題目範例 2：兩種回文排列都不大於 target
    #[test]
    fn case2() {
        assert_eq!(run("baba", "bbaa"), "");
    }

    // 奇數長度：中間固定放單數次的 'c'，只有 "abcba" / "bacab" 兩個回文排列
    #[test]
    fn case3() {
        assert_eq!(run("aabcb", "abaca"), "abcba");
    }

    // s 根本排不出回文（三個字元都是奇數個）
    #[test]
    fn case4() {
        assert_eq!(run("abc", "aaa"), "");
    }

    // 全部一樣的字元：唯一回文等於 target，不能取等號
    #[test]
    fn case5() {
        assert_eq!(run("aaa", "aaa"), "");
    }

    // 唯一回文就已經比 target 大
    #[test]
    fn case6() {
        assert_eq!(run("aaa", "aab"), "");
    }

    #[test]
    fn case7() {
        assert_eq!(run("aab", "aab"), "aba");
    }

    // 要退回更前面的位置才換得到更大的字元
    #[test]
    fn case8() {
        assert_eq!(run("aabb", "abbb"), "baab");
    }

    // matched < half：第 matched 位本身也要能放更大的字元，否則會給出偏大的 "baab"
    #[test]
    fn case9() {
        assert_eq!(run("aabb", "aabb"), "abba");
    }

    fn all_words(alphabet: &[u8], len: usize) -> Vec<String> {
        let mut out = vec![String::new()];
        for _ in 0..len {
            out = out
                .iter()
                .flat_map(|p| {
                    alphabet
                        .iter()
                        .map(move |&c| format!("{p}{}", c as char))
                        .collect::<Vec<_>>()
                })
                .collect();
        }
        out
    }

    // 對照窮舉解，掃過長度 3~4 的所有 (s, target) 組合
    #[test]
    fn brute_force_sweep() {
        for len in [1usize, 3, 4, 5] {
            let words = all_words(b"abc", len);
            for s in &words {
                for t in &words {
                    assert_eq!(run(s, t), brute(s, t), "s={s} target={t}");
                }
            }
        }
    }
}
