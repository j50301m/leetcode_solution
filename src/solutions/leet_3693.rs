struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32, costs: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut min_cost = vec![i32::MAX; n + 1];
        min_cost[0] = 0;
        for step in 1..n + 1 {
            let mut best = i32::MAX;
            for jump in 1..=3 {
                let cost = if step > jump {
                    min_cost[step - jump] + costs[step - 1] + (jump * jump) as i32
                } else {
                    min_cost[0] + costs[step - 1] + (jump * jump) as i32
                };
                best = best.min(cost);
            }
            min_cost[step] = best;
        }

        min_cost[n]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::climb_stairs(4, vec![1, 2, 3, 4]), 13);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::climb_stairs(4, vec![5, 1, 6, 2]), 11);
    }
}
