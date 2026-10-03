struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let idx = nums.partition_point(|&x| x < target);
        if nums.get(idx) == Some(&target) {
            return idx as i32;
        }
        return -1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    }
}
