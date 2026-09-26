struct Solution {}

impl Solution {
    pub fn smallest_index(nums: Vec<i32>) -> i32 {
        for (idx, &num) in nums.iter().enumerate() {
            let mut sum = 0;
            let mut num = num;
            let idx = idx as i32;
            while num > 0 {
                sum += num % 10;
                num /= 10;
            }

            if sum == idx {
                return idx;
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::smallest_index(vec![1, 3, 2]), 2);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::smallest_index(vec![1, 10, 11]), 1);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::smallest_index(vec![1, 2, 3]), -1);
    }

    // 0 的位數和是 0，剛好等於 index 0
    #[test]
    fn zero_at_index_zero() {
        assert_eq!(Solution::smallest_index(vec![0]), 0);
    }

    // 多個符合時要回傳最小的 index
    #[test]
    fn multiple_matches() {
        assert_eq!(Solution::smallest_index(vec![5, 1, 2, 3]), 1);
    }

    // 多位數：1000 -> 1, 11 -> 2
    #[test]
    fn multi_digit() {
        assert_eq!(Solution::smallest_index(vec![9, 1000, 11]), 1);
    }
}
