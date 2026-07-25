struct Solution;

impl Solution {
    pub fn max_product(mut n: i32) -> i32 {
        let mut first_largest = 0;
        let mut second_largest = 0;

        while n > 0 {
            let remainder = n % 10;
            n /= 10;
            if remainder > second_largest {
                second_largest = remainder;
                if second_largest > first_largest {
                    (first_largest, second_largest) = (second_largest, first_largest);
                }
            }
        }
        first_largest * second_largest
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let n = 124;
        let result = Solution::max_product(n);
        assert_eq!(result, 8);
    }
}
