struct Solution {}

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = citations.len();

        while left < right {
            let mid = left + (right - left) / 2;

            if citations[mid] >= (citations.len() - mid) as i32 {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        (citations.len() - left) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::h_index(vec![0, 1, 3, 5, 6]), 3);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::h_index(vec![100]), 1);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::h_index(vec![0]), 0);
    }

    #[test]
    fn case4() {
        assert_eq!(Solution::h_index(vec![3, 3, 3]), 3);
    }
}
