struct Solution;

impl Solution {
    pub fn longest_subsequence(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for &num in nums.iter() {
            ans ^= num;
        }

        if ans != 0 {
            return nums.len() as i32;
        }

        if nums.iter().any(|&x| x != 0) {
            return (nums.len() - 1) as i32;
        }

        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        // 1^2^3 = 0，拿掉任一個都可以
        assert_eq!(Solution::longest_subsequence(vec![1, 2, 3]), 2);
    }

    #[test]
    fn case_2() {
        // 全是 0，怎麼挑都是 0
        assert_eq!(Solution::longest_subsequence(vec![0, 0, 0]), 0);
    }

    #[test]
    fn case_3() {
        // 2^3^4 = 5 != 0，整個陣列都能用
        assert_eq!(Solution::longest_subsequence(vec![2, 3, 4]), 3);
    }

    #[test]
    fn case_4() {
        // 非零元素在最後面，答案還是 n-1（不是 n-i）
        assert_eq!(Solution::longest_subsequence(vec![0, 0, 3, 3]), 3);
    }

    #[test]
    fn case_5() {
        // 非零元素在最前面
        assert_eq!(Solution::longest_subsequence(vec![1, 1]), 1);
    }

    #[test]
    fn case_6() {
        assert_eq!(Solution::longest_subsequence(vec![5]), 1);
    }

    #[test]
    fn case_7() {
        assert_eq!(Solution::longest_subsequence(vec![0]), 0);
    }

    #[test]
    fn case_8() {
        // 混了 0 但總 XOR != 0
        assert_eq!(Solution::longest_subsequence(vec![0, 0, 0, 7]), 4);
    }
}
