use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    pub fn word_pattern_match(pattern: String, s: String) -> bool {
        let mut map: HashMap<u8, &str> = HashMap::new();
        let mut used: HashSet<&str> = HashSet::new();
        Self::backtrack(pattern.as_bytes(), 0, s.as_str(), 0, &mut map, &mut used)
    }

    fn backtrack<'a>(
        pattern: &[u8],
        p_idx: usize,
        s: &'a str,
        s_idx: usize,
        map: &mut HashMap<u8, &'a str>,
        used: &mut HashSet<&'a str>,
    ) -> bool {
        if p_idx == pattern.len() && s_idx == s.len() {
            return true;
        }

        if p_idx == pattern.len() || s_idx == s.len() {
            return false;
        }

        let c = pattern[p_idx];

        if let Some(&w) = map.get(&c) {
            return s[s_idx..].starts_with(w)
                && Self::backtrack(pattern, p_idx + 1, s, s_idx + w.len(), map, used);
        }

        for end in (s_idx + 1)..=s.len() {
            let candidate = &s[s_idx..end];

            if used.contains(candidate) {
                continue;
            }

            map.insert(c, candidate);
            used.insert(candidate);

            if Self::backtrack(pattern, p_idx + 1, s, end, map, used) {
                return true;
            }

            map.remove(&c);
            used.remove(candidate);
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let result = Solution::word_pattern_match("abab".to_string(), "redblueredblue".to_string());
        assert!(result);
    }

    #[test]
    fn case_2() {
        let result = Solution::word_pattern_match("aaaa".to_string(), "asdasdasdasd".to_string());
        assert!(result);
    }

    #[test]
    fn case_3() {
        let result = Solution::word_pattern_match("aabb".to_string(), "xyzabcxzyabc".to_string());
        assert!(!result);
    }
}
