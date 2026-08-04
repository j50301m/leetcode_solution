use std::collections::BTreeSet;

struct Solution;

impl Solution {
    pub fn find_missing_elements(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        nums.sort();

        let min = nums[0];
        let max = nums[n - 1];
        let mut set: BTreeSet<i32> = (min..max).collect();
        for num in nums.iter() {
            set.remove(num);
        }

        set.into_iter().collect::<Vec<i32>>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(vec![3], Solution::find_missing_elements(vec![1, 4, 2, 5]));
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Vec::<i32>::new(),
            Solution::find_missing_elements(vec![7, 8, 6, 9])
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(vec![2, 3, 4], Solution::find_missing_elements(vec![5, 1]));
    }
}
