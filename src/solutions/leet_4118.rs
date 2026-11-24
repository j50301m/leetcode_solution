struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 3 {
            return -1;
        }
        const BIG: i32 = 100000;
        let max_value = *nums.iter().max().unwrap() as usize + n;

        let mut left_dp = vec![vec![BIG; max_value + 1]; n];
        for i in 1..max_value + 1 {
            left_dp[0][i] = (nums[0] - i as i32).abs();
        }
        for peek in 1..n - 1 {
            let mut best_cost = BIG;
            for val in 1..max_value + 1 {
                left_dp[peek][val] = best_cost + (nums[peek] - val as i32).abs();
                best_cost = best_cost.min(left_dp[peek - 1][val]);
            }
        }

        let mut right_dp = vec![vec![BIG; max_value + 1]; n];
        for i in (1..max_value + 1).rev() {
            right_dp[n - 1][i] = (nums[n - 1] - i as i32).abs();
        }
        for peek in (1..n - 1).rev() {
            let mut best_cost = BIG;
            for val in 1..max_value + 1 {
                right_dp[peek][val] = best_cost + (nums[peek] - val as i32).abs();
                best_cost = best_cost.min(right_dp[peek + 1][val]);
            }
        }

        let mut best_cost = BIG;
        for peek in 1..n - 1 {
            for val in 1..max_value + 1 {
                let cost =
                    left_dp[peek][val] + right_dp[peek][val] - (nums[peek] - val as i32).abs();
                best_cost = best_cost.min(cost);
            }
        }

        best_cost
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // 題目範例
    #[test]
    fn example1() {
        assert_eq!(Solution::min_operations(vec![2, 2, 3]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::min_operations(vec![9, 4, 9]), 6);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::min_operations(vec![5, 7, 7, 4]), 1);
    }

    #[test]
    fn example4_too_short() {
        assert_eq!(Solution::min_operations(vec![1, 2]), -1);
    }

    // 邊界：長度 1
    #[test]
    fn single_element() {
        assert_eq!(Solution::min_operations(vec![1]), -1);
    }

    // 全部是 1：兩側不能降到 0，只能把中間往上推
    #[test]
    fn all_ones_len3() {
        assert_eq!(Solution::min_operations(vec![1, 1, 1]), 1);
    }

    // [1,1,1,1] -> [1,2,3,1]，成本 0+1+2+0
    #[test]
    fn all_ones_len4() {
        assert_eq!(Solution::min_operations(vec![1, 1, 1, 1]), 3);
    }

    // 原本嚴格遞減：peak 不能在 index 0
    #[test]
    fn strictly_decreasing() {
        assert_eq!(Solution::min_operations(vec![4, 3, 2, 1]), 2);
    }

    // 中間平台：[1,5,5,5,1] -> [1,5,6,5,1]，成本 1
    #[test]
    fn plateau() {
        assert_eq!(Solution::min_operations(vec![1, 5, 5, 5, 1]), 1);
    }

    #[test]
    fn mixed() {
        assert_eq!(Solution::min_operations(vec![3, 1, 2, 5, 1]), 4);
    }
}
