struct Solution {}

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut min_len = usize::MAX;
        let mut sum = 0;
        let mut left = 0;
        for right in 0..n {
            sum += nums[right];
            while sum >= target && right >= left {
                min_len = min_len.min(right - left + 1);
                sum -= nums[left];
                left += 1;
            }
        }

        if min_len == usize::MAX {
            return 0;
        }
        min_len as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::min_sub_array_len(11, vec![1, 1, 1]), 0);
    }
}
