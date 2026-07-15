pub struct Solution;

impl Solution {
    pub fn subarray_lcm(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        for start in 0..nums.len() {
            let mut curr_lcm = 1;
            for end in start..nums.len() {
                let tmp = Self::lcm(curr_lcm, nums[end]);
                if tmp == k {
                    count += 1;
                }
                if tmp > k {
                    break;
                }
                curr_lcm = tmp;
            }
        }

        count
    }

    fn lcm(a: i32, b: i32) -> i32 {
        a / Self::gcd(a, b) * b
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let tmp = a % b;
            a = b;
            b = tmp;
        }
        a
    }
}

#[cfg(test)]
mod test {
    use std::assert_eq;

    use super::*;

    #[test]
    fn case_1() {
        let nums = vec![3, 6, 2, 7, 1];
        let k = 6;
        let result = Solution::subarray_lcm(nums, k);
        assert_eq!(result, 4);
    }

    #[test]
    fn case_2() {
        let nums = vec![3];
        let k = 2;
        let result = Solution::subarray_lcm(nums, k);
        assert_eq!(result, 0);
    }
}
