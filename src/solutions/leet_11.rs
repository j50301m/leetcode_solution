pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max_area = 0;
        while left < right {
            let h = if height[left] < height[right] {
                height[left]
            } else {
                height[right]
            };

            let area = h * (right - left) as i32;
            if area > max_area {
                max_area = area;
            }

            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max_area
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let result = Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]);
        assert_eq!(result, 49);
    }

    #[test]
    fn case_2() {
        let result = Solution::max_area(vec![1, 1]);
        assert_eq!(result, 1);
    }
}
