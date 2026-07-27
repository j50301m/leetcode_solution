struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut prefix: Vec<i32> = vec![0; n];
        let mut subfix: Vec<i32> = vec![0; n];
        for i in 0..n {
            if i == 0 {
                prefix[i] = nums[i];
                continue;
            }
            prefix[i] = prefix[i - 1] * nums[i];
        }

        for i in (0..n).rev() {
            if i == n - 1 {
                subfix[i] = nums[i];
                continue;
            }
            subfix[i] = subfix[i + 1] * nums[i];
        }

        let mut result = vec![0; n];
        for i in 0..n {
            let l = if i == 0 {
                1
            } else {
                *prefix.get(i - 1).unwrap()
            };
            let r = if i == n - 1 {
                1
            } else {
                *subfix.get(i + 1).unwrap()
            };
            result[i] = l * r;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let nums = vec![1, 2, 3, 4];
        let result = Solution::product_except_self(nums);
        assert_eq!(result, vec![24, 12, 8, 6]);
    }
}
