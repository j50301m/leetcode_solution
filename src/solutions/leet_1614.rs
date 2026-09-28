struct Solution {}

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let bytes = s.as_bytes();

        let mut parentheses = 0;
        let mut max_cnt = 0;
        bytes.iter().for_each(|&b| {
            if b == b'(' {
                parentheses += 1;
            } else if b == b')' {
                parentheses -= 1;
            }

            max_cnt = max_cnt.max(parentheses);
        });

        max_cnt
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::max_depth("(1+(2*3)+((8)/4))+1".to_string()), 3);
    }
}
