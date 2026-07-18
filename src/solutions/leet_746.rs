struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut sum = 0;
        for i in (0..cost.len()).step_by(2) {
            let a = cost[i];
            let b = *cost.get(i + 1).unwrap_or(&0);

            if a <= b {
                sum += a;
            } else {
                sum += b;
            }
        }

        sum
    }
}

// Input: cost = [10,15,20]
// Output: 15
// Explanation: You will start at index 1.
// - Pay 15 and climb two steps to reach the top.
// The total cost is 15.

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let nums = vec![10, 15, 20];
        let result = Solution::min_cost_climbing_stairs(nums);
        assert_eq!(result, 15);
    }
}
