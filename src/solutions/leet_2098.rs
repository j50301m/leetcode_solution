use std::cmp::Reverse;

struct Solution {}

impl Solution {
    pub fn largest_even_sum(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut nums: Vec<i64> = nums.iter().map(|&x| x as i64).collect();
        nums.sort_by_key(|&x| Reverse(x));
        let sum = nums[0..k].iter().sum();
        if sum % 2 == 0 {
            return sum;
        }

        let mut even_sum = -1;
        if let Some(&last_odd) = nums[..k].iter().rev().find(|&x| *x % 2 != 0) {
            if let Some(&even) = nums[k..].iter().find(|&x| *x % 2 == 0) {
                even_sum = sum - last_odd + even;
            }
        }

        if let Some(&last_even) = nums[..k].iter().rev().find(|&x| *x % 2 == 0) {
            if let Some(&odd) = nums[k..].iter().find(|&x| *x % 2 != 0) {
                even_sum = even_sum.max(sum - last_even + odd);
            }
        }

        even_sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::largest_even_sum(vec![4, 1, 5, 3, 1], 3), 12);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::largest_even_sum(vec![4, 6, 2], 3), 12);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::largest_even_sum(vec![1, 3, 5], 3), -1);
    }

    #[test]
    fn case4() {
        assert_eq!(Solution::largest_even_sum(vec![4, 6, 2, 3, 5], 4), 18);
    }
}
