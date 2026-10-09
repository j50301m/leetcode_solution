struct Solution {}

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut balance: i32 = 0;
        let mut cnt = 0;
        for &b in bytes {
            match b {
                b'(' => {
                    if balance < 0 {
                        let x = balance.abs();
                        cnt += x / 2 + 2 * (x % 2);
                        balance = 0;
                    }
                    if balance % 2 == 1 {
                        cnt += 1;
                        balance -= 1;
                    }
                    balance += 2;
                }
                _ => {
                    balance -= 1;
                }
            }
        }
        if balance < 0 {
            let x = balance.abs();
            cnt += x / 2 + 2 * (x % 2);
        } else {
            cnt += balance;
        }
        cnt
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::min_insertions("(()))".to_string()), 1);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::min_insertions("())".to_string()), 0);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::min_insertions("))())(".to_string()), 3);
    }

    #[test]
    fn case4() {
        assert_eq!(Solution::min_insertions(")))))))".to_string()), 5);
    }

    #[test]
    fn case5() {
        assert_eq!(Solution::min_insertions("(()))(()))()())))".to_string()), 4);
    }

    #[test]
    fn case6() {
        assert_eq!(
            Solution::min_insertions(
                "(((()(()((())))(((()())))()())))(((()(()()((()()))".to_string()
            ),
            31
        );
    }
}
