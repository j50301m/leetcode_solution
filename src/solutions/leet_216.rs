struct Solution {}

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let digits = (1..=9).collect::<Vec<usize>>();
        let mut results = Vec::new();
        let mut memo = Vec::new();
        Self::dfs(
            &digits,
            0,
            0,
            0,
            k as usize,
            n as usize,
            &mut memo,
            &mut results,
        );
        results
    }

    fn dfs(
        digits: &Vec<usize>,
        curr_k: usize,
        curr_idx: usize,
        curr_sum: usize,
        k: usize,
        target: usize,
        memo: &mut Vec<i32>,
        results: &mut Vec<Vec<i32>>,
    ) {
        if curr_k == k {
            if target == curr_sum {
                results.push(memo.clone());
            }
            return;
        }

        if curr_sum > target {
            return;
        }

        for i in curr_idx..digits.len() {
            memo.push((i + 1) as i32);
            Self::dfs(
                digits,
                curr_k + 1,
                i + 1,
                curr_sum + digits[i],
                k,
                target,
                memo,
                results,
            );
            memo.pop();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::combination_sum3(3, 7), vec![vec![1, 2, 4]]);
    }
}
