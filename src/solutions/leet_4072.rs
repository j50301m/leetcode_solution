struct Solution {}

impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut plus_0 = vec![i64::MIN; n + 1];
        let mut minus_0 = vec![i64::MIN; n + 1];
        let mut plus_1 = vec![i64::MIN; n + 1];
        let mut minus_1 = vec![i64::MIN; n + 1];

        let mut best = i64::MIN;
        for (i, &num) in nums.iter().enumerate() {
            let num = num as i64;
            plus_0[i + 1] = num.max(minus_0[i].saturating_add(num));
            minus_0[i + 1] = plus_0[i].saturating_sub(num);
            plus_1[i + 1] = (minus_1[i].saturating_add(num)).max(plus_0[i]);
            minus_1[i + 1] = (plus_1[i].saturating_sub(num)).max(minus_0[i]);
            best = plus_0[i + 1]
                .max(plus_1[i + 1])
                .max(minus_0[i + 1])
                .max(minus_1[i + 1])
                .max(best);
        }

        best
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::max_alternating_sum(vec![5, -5, 1]), 11);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::max_alternating_sum(vec![10, -5, -100]), 110);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::max_alternating_sum(vec![94, 80, 20]), 94);
    }
}
