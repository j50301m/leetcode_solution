struct Solution {}

impl Solution {
    pub fn count_servers(n: i32, mut logs: Vec<Vec<i32>>, x: i32, queries: Vec<i32>) -> Vec<i32> {
        let n = n as usize;
        logs.sort_by_key(|vec| vec[1]);
        let mut queries = queries
            .into_iter()
            .enumerate()
            .map(|(idx, end_time)| (idx, end_time as usize))
            .collect::<Vec<(usize, usize)>>();
        queries.sort_by_key(|&(_idx, end_time)| end_time);

        let mut counts = vec![0; n + 1];
        let mut left = 0;
        let mut right = 0;
        let mut ans = vec![0; queries.len()];
        let mut active_servers = 0;
        for &(idx, end) in queries.iter() {
            let start = end - x as usize;
            while right < logs.len() && logs[right][1] as usize <= end {
                counts[logs[right][0] as usize] += 1;
                if counts[logs[right][0] as usize] == 1 {
                    active_servers += 1;
                }
                right += 1;
            }
            while left < right && (logs[left][1] as usize) < start {
                counts[logs[left][0] as usize] -= 1;
                if counts[logs[left][0] as usize] == 0 {
                    active_servers -= 1;
                }
                left += 1;
            }

            ans[idx] = n as i32 - active_servers;
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
            Solution::count_servers(3, vec![vec![1, 3], vec![2, 6], vec![1, 5]], 5, vec![10, 11]),
            vec![1, 2]
        );
    }
}
