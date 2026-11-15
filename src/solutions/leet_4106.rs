struct Solution {}

impl Solution {
    pub fn find_mismatched_segments(mut s1: String, mut s2: String) -> Vec<Vec<String>> {
        s1.push('_');
        s2.push('_');
        let b1 = s1.as_bytes();
        let b2 = s2.as_bytes();
        let mut left = -1i32;
        let mut ans = Vec::new();
        for right in 0..b1.len() {
            if b1[right] == b2[right] {
                let start = (left + 1) as usize;
                if start == right {
                    left = right as i32;
                    continue;
                }
                if let (Ok(ans1), Ok(ans2)) = (
                    str::from_utf8(&b1[start..right]),
                    str::from_utf8(&b2[start..right]),
                ) {
                    ans.push(vec![ans1.to_string(), ans2.to_string()]);
                };
                left = right as i32;
            }
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::find_mismatched_segments("earth".to_string(), "empty".to_string()),
            vec![
                vec!["ar".to_string(), "mp".to_string()],
                vec!["h".to_string(), "y".to_string()]
            ]
        );
    }
}
