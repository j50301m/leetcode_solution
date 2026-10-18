use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let mut sorted = nums.clone();
        sorted.sort_by_cached_key(|&a| {
            let mut a_sum = 0;
            let mut _a = a;
            while _a > 0 {
                a_sum += _a % 10;
                _a /= 10;
            }
            (a_sum, a)
        });

        let mut map = HashMap::new();
        for (idx, &val) in sorted.iter().enumerate() {
            map.insert(val, idx);
        }

        let n = nums.len();
        let mut visited = vec![false; n];
        let mut swaps = 0;

        for start in 0..n {
            if visited[start] {
                continue;
            }

            let mut cycle_length = 0;
            let mut current = start;
            while !visited[current] {
                visited[current] = true;
                current = map[&nums[current]];
                cycle_length += 1;
            }
            swaps += cycle_length - 1;
        }

        swaps as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::min_swaps(vec![37, 100]), 1);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::min_swaps(vec![22, 14, 33, 7]), 0);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::min_swaps(vec![193644115, 569655076, 91600837]), 2);
    }

    #[test]
    fn case4() {
        assert_eq!(
            Solution::min_swaps(vec![277993448, 416038674, 616955867, 539372283]),
            3
        );
    }
}
