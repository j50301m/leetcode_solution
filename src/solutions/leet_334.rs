struct Solution;

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut small = i32::MAX;
        let mut mid = i32::MAX;
        for n in nums {
            if n <= small {
                small = n;
            } else if n <= mid {
                mid = n;
            } else {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let nums = vec![2, 1, 5, 0, 4, 6];
        let result = Solution::increasing_triplet(nums);
        assert_eq!(result, true)
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::increasing_triplet(vec![1, 1, 1]), false);
    }
}
