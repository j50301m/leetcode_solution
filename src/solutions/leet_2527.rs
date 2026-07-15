pub struct Solution;

impl Solution {
    pub fn xor_beauty(nums: Vec<i32>) -> i32 {
        let acc = nums.iter().fold(0, |acc, x| acc ^ x);
        acc
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let result = Solution::xor_beauty(vec![1, 4]);
        assert_eq!(result, 5);
    }

    #[test]
    fn case_2() {
        let result = Solution::xor_beauty(vec![15, 45, 20, 2, 34, 35, 5, 44, 32, 30]);
        assert_eq!(result, 34);
    }
}
