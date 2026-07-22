struct Solution;

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let mut max = 0;
        let mut result = vec![false; candies.len()];
        for &ele in candies.iter() {
            if ele > max {
                max = ele;
            }
        }

        let success_candidate = max - extra_candies;
        for i in 0..candies.len() {
            if candies[i] >= success_candidate {
                result[i] = true
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let candies = vec![2, 3, 5, 1, 3];
        let extra_candies = 3;
        let result = Solution::kids_with_candies(candies, extra_candies);
        assert_eq!(result, vec![true, true, true, false, true])
    }
}
