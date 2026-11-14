struct Solution {}

impl Solution {
    pub fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        let (l, r) = (l as usize, r as usize);
        let n = nums.len();

        let mut prefix_sum = vec![0; n + 1];
        for i in 0..n {
            prefix_sum[i + 1] = prefix_sum[i] + nums[i];
        }

        let mut min_sum = i32::MAX;
        for range in l..=r {
            for i in range..n + 1 {
                let sum = prefix_sum[i] - prefix_sum[i - range];
                if sum <= 0 {
                    continue;
                }
                min_sum = min_sum.min(sum);
            }
        }
        if min_sum == i32::MAX {
            return -1;
        }

        min_sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::minimum_sum_subarray(vec![3, -2, 1, 4], 2, 3), 1);
    }
}
