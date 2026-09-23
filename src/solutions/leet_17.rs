use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return Vec::new();
        }

        let char_map = [
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
            vec!['j', 'k', 'l'],
            vec!['m', 'n', 'o'],
            vec!['p', 'q', 'r', 's'],
            vec!['t', 'u', 'v'],
            vec!['w', 'x', 'y', 'z'],
        ];

        let bytes = digits.as_bytes();
        let mut queue = VecDeque::new();
        let mut results = Vec::new();
        queue.push_front((0usize, String::new())); // (digits index, building string
        while let Some((idx, curr_str)) = queue.pop_back() {
            if idx == bytes.len() {
                results.push(curr_str);
                continue;
            }

            let char_idx = (bytes[idx] - b'0') as usize - 2;
            for &c in char_map[char_idx].iter() {
                let mut new_str = curr_str.clone();
                new_str.push(c);
                queue.push_front((idx + 1, new_str));
            }
        }

        results
    }

    // 回溯版：一條字串重複利用，push 進去 -> 往下遞迴 -> pop 出來還原現場
    pub fn letter_combinations_backtrack(digits: String) -> Vec<String> {
        const CHAR_MAP: [&str; 8] = ["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];

        fn dfs(bytes: &[u8], idx: usize, curr: &mut String, results: &mut Vec<String>) {
            if idx == bytes.len() {
                results.push(curr.clone());
                return;
            }
            for c in CHAR_MAP[(bytes[idx] - b'2') as usize].chars() {
                curr.push(c); // 做選擇
                dfs(bytes, idx + 1, curr, results);
                curr.pop(); // 撤銷選擇 <- 這就是 backtracking
            }
        }

        if digits.is_empty() {
            return Vec::new();
        }
        let mut results = Vec::new();
        dfs(digits.as_bytes(), 0, &mut String::new(), &mut results);
        results
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // 兩個版本都跑，順便互相驗證輸出一致
    fn run(d: &str) -> Vec<String> {
        let bfs = Solution::letter_combinations(d.to_string());
        let backtrack = Solution::letter_combinations_backtrack(d.to_string());
        assert_eq!(bfs, backtrack, "BFS 版與回溯版輸出不一致: {d:?}");
        backtrack
    }

    #[test]
    fn case1() {
        assert_eq!(
            run("23"),
            vec![
                "ad".to_string(),
                "ae".to_string(),
                "af".to_string(),
                "bd".to_string(),
                "be".to_string(),
                "bf".to_string(),
                "cd".to_string(),
                "ce".to_string(),
                "cf".to_string()
            ]
        );
    }

    #[test]
    fn empty() {
        assert_eq!(run(""), Vec::<String>::new());
    }

    #[test]
    fn single_digit_four_letters() {
        assert_eq!(run("7"), vec!["p", "q", "r", "s"]);
    }

    #[test]
    fn three_digits_count_and_content() {
        let got = run("234");
        assert_eq!(got.len(), 27);
        assert_eq!(got[0], "adg");
        assert_eq!(got[26], "cfi");
        let mut sorted = got.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), 27, "combinations must be unique");
    }

    #[test]
    fn max_size_9999() {
        // 9 = wxyz, 4 letters each -> 4^4
        assert_eq!(run("9999").len(), 256);
    }
}
