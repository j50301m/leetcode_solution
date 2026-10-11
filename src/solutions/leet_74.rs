struct Solution {}

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut left = 0;
        let mut right = matrix.len();

        while left < right {
            let mid = left + (right - left) / 2;

            if matrix[mid][0] > target {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        if left == 0 {
            return false;
        }
        let start = left - 1;

        left = 0;
        right = matrix[start].len();

        while left < right {
            let mid = left + (right - left) / 2;

            if matrix[start][mid] >= target {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        if left == matrix[start].len() {
            return false;
        }
        let end = left;

        matrix[start][end] == target
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn example_matrix() -> Vec<Vec<i32>> {
        vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]]
    }

    #[test]
    fn case1_target_in_middle_of_row() {
        assert_eq!(Solution::search_matrix(example_matrix(), 3), true);
    }

    #[test]
    fn case2_target_not_found() {
        assert_eq!(Solution::search_matrix(example_matrix(), 13), false);
    }

    #[test]
    fn case3_target_is_first_of_row() {
        assert_eq!(Solution::search_matrix(example_matrix(), 10), true);
    }

    #[test]
    fn case4_target_is_last_element() {
        assert_eq!(Solution::search_matrix(example_matrix(), 60), true);
    }

    #[test]
    fn case5_target_smaller_than_all() {
        assert_eq!(Solution::search_matrix(example_matrix(), 0), false);
    }

    #[test]
    fn case6_target_larger_than_all() {
        assert_eq!(Solution::search_matrix(example_matrix(), 100), false);
    }

    #[test]
    fn case7_single_element() {
        assert_eq!(Solution::search_matrix(vec![vec![1]], 1), true);
    }

    #[test]
    fn case8_single_row() {
        assert_eq!(Solution::search_matrix(vec![vec![1, 3]], 3), true);
    }
}
