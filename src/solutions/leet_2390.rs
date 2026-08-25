struct Solution;

impl Solution {
    pub fn remove_stars(s: String) -> String {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut stack: Vec<char> = Vec::with_capacity(n);
        for i in 0..n {
            let c = bytes[i] as char;
            if c == '*' {
                stack.pop();
                continue;
            }
            stack.push(c);
        }

        stack
            .into_iter()
            .fold(String::new(), |acc, x| format!("{}{}", acc, x))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::remove_stars("leet**cod*e".to_string()),
            "lecoe".to_string()
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::remove_stars("erase*****".to_string()),
            "".to_string(),
        )
    }
}
