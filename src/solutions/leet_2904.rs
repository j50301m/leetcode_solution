struct Solution;

impl Solution {
    pub fn shortest_beautiful_substring(s: String, k: i32) -> String {
        let bytes = s.as_bytes();
        let mut left = 0;
        let mut found = 0;
        let mut best: Option<&str> = None;

        for i in 0..bytes.len() {
            if bytes[i] == b'1' {
                found += 1;
            }

            while found > k {
                if bytes[left] == b'1' {
                    found -= 1;
                }
                left += 1;
            }

            // 最小窗口頭一定是 '1'，先把左邊多餘的 '0' 縮掉
            while found == k && bytes[left] == b'0' {
                left += 1;
            }

            if found == k {
                let cand = &s[left..=i];
                if best.map_or(true, |b| (cand.len(), cand) < (b.len(), b)) {
                    best = Some(cand);
                }
            }
        }

        best.unwrap_or("").to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::shortest_beautiful_substring("100011001".to_string(), 3),
            "11001".to_string()
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::shortest_beautiful_substring("1011".to_string(), 2),
            "11".to_string()
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            Solution::shortest_beautiful_substring("000".to_string(), 1),
            "".to_string()
        );
    }

    // 同長度時要取字典序最小："1011" < "1101"
    #[test]
    fn case4() {
        assert_eq!(
            Solution::shortest_beautiful_substring("1101001011".to_string(), 3),
            "1011".to_string()
        );
    }

    #[test]
    fn case5() {
        assert_eq!(
            Solution::shortest_beautiful_substring("11".to_string(), 1),
            "1".to_string()
        );
    }
}
