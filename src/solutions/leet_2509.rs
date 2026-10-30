struct Solution {}

impl Solution {
    pub fn cycle_length_queries(_n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = Vec::with_capacity(queries.len());
        for query in queries.iter() {
            let (mut a, mut b) = (query[0] as usize, query[1] as usize);
            let mut step = 0;
            while a != b {
                if a > b {
                    a /= 2;
                } else {
                    b /= 2;
                }
                step += 1;
            }
            ans.push(step + 1);
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::cycle_length_queries(3, vec![vec![5, 3], vec![4, 7], vec![2, 3]]),
            vec![4, 5, 3]
        );
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::cycle_length_queries(2, vec![vec![1, 2]]), vec![2]);
    }

    #[test]
    fn one_is_ancestor_of_other() {
        // 8 -> 4 -> 2, depth gap = 2
        assert_eq!(Solution::cycle_length_queries(4, vec![vec![8, 2]]), vec![3]);
    }

    #[test]
    fn different_depths_meet_at_root() {
        // 8 -> 4 -> 2 -> 1 <- 3
        assert_eq!(Solution::cycle_length_queries(4, vec![vec![8, 3]]), vec![5]);
    }
}
