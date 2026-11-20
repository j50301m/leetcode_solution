struct Solution {}

impl Solution {
    pub fn max_selected_elements(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let n = nums.len();
        let mut counts = vec![0; 1000001];
        let mut max_len = 0;
        for i in 0..n {
            let num = nums[i] as usize;
            counts[num + 1] = counts[num] + 1;
            counts[num] = counts[num - 1] + 1;
            max_len = max_len.max(counts[num + 1]).max(counts[num]);
        }

        max_len
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::max_selected_elements(vec![2, 1, 5, 1, 1]), 3);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::max_selected_elements(vec![1, 4, 7, 10]), 1);
    }

    #[test]
    fn case3() {
        assert_eq!(
            Solution::max_selected_elements(vec![
                8, 10, 6, 12, 9, 12, 2, 3, 13, 19, 11, 18, 10, 16
            ]),
            8
        );
    }
}
