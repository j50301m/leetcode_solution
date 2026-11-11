struct Solution {}

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut used = 0;
        let mut unused = 0;
        let mut max = 0;
        for num in nums {
            if num == 1 {
                used += 1;
                unused += 1;
            } else {
                used = unused + 1;
                unused = 0;
            }
            max = max.max(used).max(unused);
        }
        max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0]), 4);
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]),
            4
        );
    }
}
