struct Solution {}

impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        let mut left = 0usize;
        let mut right = arr.len();

        while left < right {
            let mid = left + (right - left) / 2;
            let miss_count = arr[mid] - mid as i32 - 1;

            if miss_count >= k {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left as i32 + k
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::find_kth_positive(vec![2, 3, 4, 7, 11], 1), 1);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::find_kth_positive(vec![1, 2], 1), 3);
    }
}
