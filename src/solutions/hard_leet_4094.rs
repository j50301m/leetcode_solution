struct Solution {}

impl Solution {
    pub fn count_staircase(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let mut value_to_id = vec![-1i32; 100001];
        let mut map = vec![0usize; n];
        let mut m = 0;
        for i in 0..n {
            let value = nums[i] as usize;
            if value_to_id[value] == -1 {
                value_to_id[value] = m;
                m += 1;
            }
            map[i] = value_to_id[value] as usize;
        }

        let mut state = vec![(0u16, 0u16); m as usize];
        let mut ans = 0;
        for start in 0..n {
            let mut distinct_count = 0;
            let mut touch_values = Vec::new();
            for i in start..n {
                let (count, rank) = &mut state[map[i]];
                if *rank == 0 {
                    distinct_count += 1;
                    *rank = distinct_count;
                    touch_values.push(map[i]);
                }
                *count += 1;

                let match_length = (distinct_count as usize + 1) * distinct_count as usize / 2;
                if match_length > n - start {
                    break;
                }
                if *count > *rank {
                    break;
                }

                // 如果當前長度 == 需要滿足的階梯長度 => 符合
                // 如果 當前長度 < match_length => 還沒到 繼續
                if (i - start + 1) == match_length {
                    ans += 1;
                }
            }

            for touch in touch_values {
                state[touch] = (0, 0);
            }
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::count_staircase(vec![5, 8, 8, 2, 2, 2]), 9);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::count_staircase(vec![1, 2, 3, 3, 3, 9]), 7);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::count_staircase(vec![2, 3, 4, 4, 3, 4]), 8);
    }
}
