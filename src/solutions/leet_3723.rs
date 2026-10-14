struct Solution {}

impl Solution {
    pub fn max_sum_of_squares(num: i32, sum: i32) -> String {
        let mut sum = sum;
        let mut s = String::new();

        for _ in 0..num {
            if sum < 9 {
                let c = (sum as u8 + b'0') as char;
                s.push(c);
                sum = 0;
            } else {
                s.push('9');
                sum -= 9;
            }
        }

        if sum > 0 {
            return String::new();
        }

        s
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::max_sum_of_squares(2, 3), "30".to_string());
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::max_sum_of_squares(2, 17), "98".to_string());
    }
}
