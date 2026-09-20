struct Solution {}

impl Solution {
    pub fn reverse_degree(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut sum = 0;
        for (idx, &b) in bytes.iter().enumerate() {
            let alphabet = (26 - (b - b'a')) as usize;
            let idx = idx + 1;
            sum += alphabet * idx;
        }

        sum as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::reverse_degree("abc".to_string()), 148);
    }
}
