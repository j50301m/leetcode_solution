struct Solution;

impl Solution {
    pub fn generate_abbreviations(word: String) -> Vec<String> {
        let mut result = Vec::new();
        Self::dfs(0, &String::new(), 0, &word, &mut result);

        result
    }

    fn dfs(pos: usize, curr_str: &String, count: i32, word: &String, result: &mut Vec<String>) {
        if pos == word.len() {
            let mut abbr = curr_str.to_string();
            if count > 0 {
                abbr += &count.to_string();
            }
            result.push(abbr);
            return;
        }

        Self::dfs(pos + 1, curr_str, count + 1, word, result);

        let mut new = curr_str.to_string();
        if count > 0 {
            new += &count.to_string();
        }
        new.push(word.as_bytes()[pos] as char);
        Self::dfs(pos + 1, &new, 0, word, result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(mut v: Vec<String>) -> Vec<String> {
        v.sort();
        v
    }

    #[test]
    fn leetcode_case() {
        let expected = [
            "4", "3d", "2r1", "2rd", "1o2", "1o1d", "1or1", "1ord", "w3", "w2d", "w1r1", "w1rd",
            "wo2", "wo1d", "wor1", "word",
        ]
        .map(String::from)
        .to_vec();
        assert_eq!(
            sorted(Solution::generate_abbreviations("word".to_string())),
            sorted(expected)
        );
    }

    #[test]
    fn single_char() {
        assert_eq!(
            sorted(Solution::generate_abbreviations("a".to_string())),
            sorted(["1", "a"].map(String::from).to_vec())
        );
    }
}
