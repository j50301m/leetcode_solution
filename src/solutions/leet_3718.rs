struct Solution;

impl Solution {
    pub fn missing_multiple(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut divisor = 1;
        nums.sort();
        for num in nums {
            if num % k != 0 {
                continue;
            }

            let d = num / k;
            if d < 0 {
                continue;
            }

            if d == divisor {
                divisor += 1;
            } else if d > divisor {
                break;
            }
        }

        divisor * k
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::missing_multiple(vec![8, 2, 3, 4, 6], 2), 10);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::missing_multiple(vec![1, 4, 7, 10, 15], 5), 5);
    }
}
