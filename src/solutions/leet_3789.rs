struct Solution {}

impl Solution {
    pub fn minimum_cost(cost1: i32, cost2: i32, cost_both: i32, need1: i32, need2: i32) -> i64 {
        let combine_cost = cost_both.min(cost1 + cost2) as i64;
        let common_needs = need1.min(need2) as i64;
        let mut cost = combine_cost * common_needs;
        if need1 > need2 {
            cost += cost1.min(cost_both) as i64 * (need1 - need2) as i64;
        } else {
            cost += cost2.min(cost_both) as i64 * (need2 - need1) as i64;
        }

        cost
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::minimum_cost(3, 2, 1, 3, 2), 3);
    }
}
