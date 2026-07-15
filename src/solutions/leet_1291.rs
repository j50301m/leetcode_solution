pub struct Solution;

impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut _low = low;
        let mut _high = high;
        // Create a vector with all digits
        let digits = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        // Find the min_window, and max_window
        let mut min_window = 1;
        let mut max_window = 1;
        while _low > 0 || _high > 0 {
            _low = _low / 10;
            if _low > 0 {
                min_window += 1;
            }

            _high = _high / 10;
            if _high > 0 {
                max_window += 1;
            }
        }

        // For loop: for each window try to get a num in [low, high], with sequential digit
        let mut result = Vec::new();
        for window in min_window..=max_window {
            for slice in digits.windows(window) {
                let num = slice.iter().fold(0, |acc, digit| acc * 10 + digit);
                if num >= low && num <= high {
                    result.push(num);
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let result = Solution::sequential_digits(100, 300);
        assert_eq!(result, vec![123, 234]);
    }

    #[test]
    fn case_2() {
        let result = Solution::sequential_digits(1000, 13000);
        assert_eq!(result, vec![1234, 2345, 3456, 4567, 5678, 6789, 12345]);
    }
}
