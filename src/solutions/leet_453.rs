struct Solution {}

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let min = nums.iter().min().unwrap();
        let mut sum = 0;
        for &num in nums.iter() {
            sum += num - min;
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::min_moves(vec![1, 2, 3]), 3);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::min_moves(vec![1, 1, 1]), 0);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::min_moves(vec![5]), 0);
    }

    #[test]
    fn one_large_value() {
        // [1,1,4] -> [2,2,4] -> [3,3,4] -> [4,4,4]: 3 moves
        assert_eq!(Solution::min_moves(vec![1, 1, 4]), 3);
    }

    #[test]
    fn negatives() {
        assert_eq!(Solution::min_moves(vec![-1, 0, 2]), 4);
    }
}
