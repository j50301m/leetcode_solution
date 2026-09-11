use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn total_numbers(digits: Vec<i32>) -> i32 {
        let n = digits.len();
        let mut set = HashSet::new();
        for i in 0..n {
            if digits[i] == 0 {
                continue;
            }
            for j in 0..n {
                if j == i {
                    continue;
                }
                for k in 0..n {
                    if k == i || k == j {
                        continue;
                    }

                    let num = digits[i] * 100 + digits[j] * 10 + digits[k];
                    if num % 2 != 0 {
                        continue;
                    }

                    set.insert(num);
                }
            }
        }

        set.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::total_numbers(vec![1, 2, 3, 4]), 12)
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::total_numbers(vec![0, 2, 2]), 2)
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::total_numbers(vec![6, 6, 6]), 1)
    }

    #[test]
    fn case4() {
        assert_eq!(Solution::total_numbers(vec![1, 3, 5]), 0)
    }

    #[test]
    fn all_zero() {
        assert_eq!(Solution::total_numbers(vec![0, 0, 0]), 0)
    }

    #[test]
    fn leading_zero_only_source_of_even() {
        assert_eq!(Solution::total_numbers(vec![0, 1, 3]), 2)
    }
}
