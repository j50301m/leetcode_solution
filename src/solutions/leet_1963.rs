struct Solution {}

impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut balance = 0;
        for &b in bytes {
            if b == b'[' {
                balance += 1;
            } else {
                if balance == 0 {
                    continue;
                }
                balance -= 1;
            }
        }
        (balance + 1) / 2 as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::min_swaps("][][".to_string()), 1);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::min_swaps("]]][[[".to_string()), 2);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::min_swaps("[[[]]]][][]][[]]][[[".to_string()), 2);
    }
}
