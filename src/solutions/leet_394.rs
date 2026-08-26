struct Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut num_stack = Vec::new();
        let mut s_stack = Vec::new();
        let mut num = 0;
        let mut curr = String::new();
        for &b in s.as_bytes() {
            match b {
                b'0'..=b'9' => num = num * 10 + (b - b'0') as usize,
                b'[' => {
                    num_stack.push(num);
                    s_stack.push(std::mem::take(&mut curr));
                    num = 0;
                }
                b']' => {
                    let n = num_stack.pop().unwrap();
                    let prefix = s_stack.pop().unwrap();
                    curr = prefix + &curr.repeat(n);
                }
                _ => curr.push(b as char),
            }
        }

        curr
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::decode_string("3[a]2[bc]".to_string()),
            "aaabcbc".to_string()
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::decode_string("3[a2[c]]".to_string()),
            "accaccacc".to_string()
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            Solution::decode_string("2[abc]3[cd]ef".to_string()),
            "abcabccdcdcdef".to_string()
        );
    }

    #[test]
    fn case4() {
        assert_eq!(
            Solution::decode_string("10[a]".to_string()),
            "aaaaaaaaaa".to_string()
        );
    }
}
