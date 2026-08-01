struct Solution;

impl Solution {
    pub fn bold_words(words: Vec<String>, s: String) -> String {
        let n = s.len();
        let mut marks: Vec<bool> = vec![false; n];
        let s_bytes = s.as_bytes();
        for word in words {
            let word_bytes = word.as_bytes();
            let word_len = word.len();
            let mut i = 0;
            while i < n - word_len {
                if word_bytes[..] == s_bytes[i..word_len + i] {
                    let _ = marks[i..word_len + i].iter_mut().map(|_| true);
                }
                i += 1;
            }
        }

        let mut result = String::new();
        for i in 0..n {
            if marks[i] && i == 0 {
                result = format!("<b>{}", s_bytes[i] as char);
            }
        }

        result
    }
}
