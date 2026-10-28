struct Solution {}

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        const XOR_LIMIT: usize = 1024;
        let mut dp = vec![[false; XOR_LIMIT]; n];
        for row in 0..m {
            for col in 0..n {
                let mut curr = [false; XOR_LIMIT];
                let val = grid[row][col] as usize;
                if row == 0 && col == 0 {
                    curr[val] = true;
                } else {
                    for case in 0..XOR_LIMIT {
                        let has_above = row > 0 && dp[col][case];
                        let has_left = col > 0 && dp[col - 1][case];
                        if has_above || has_left {
                            curr[val ^ case] = true;
                        }
                    }
                }
                dp[col] = curr;
            }
        }

        dp[n - 1]
            .iter()
            .enumerate()
            .find_map(|(idx, val)| if *val == true { Some(idx as i32) } else { None })
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::min_cost(vec![vec![1, 2], vec![3, 4]]), 6);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::min_cost(vec![vec![2, 7, 5]]), 0);
    }
}
