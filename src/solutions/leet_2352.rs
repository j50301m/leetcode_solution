struct Solution;

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut transpose = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                transpose[j][i] = grid[i][j];
            }
        }

        let mut total = 0;
        for i in 0..n {
            for j in 0..n {
                if grid[i] == transpose[j] {
                    total += 1;
                }
            }
        }

        total
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::equal_pairs(vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]]),
            1
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::equal_pairs(vec![
                vec![3, 1, 2, 2],
                vec![1, 4, 4, 5],
                vec![2, 4, 2, 2],
                vec![2, 4, 2, 2]
            ]),
            3
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::equal_pairs(vec![vec![1]]), 1);
    }

    #[test]
    fn all_same() {
        assert_eq!(Solution::equal_pairs(vec![vec![2, 2], vec![2, 2]]), 4);
    }
}
