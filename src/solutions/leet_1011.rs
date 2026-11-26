struct Solution {}

impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        // let n = weights.len();
        let mut right = weights.iter().sum();
        let &max = weights.iter().max().unwrap();
        let mut left = 1;

        while left < right {
            let mid = left + (right - left) / 2;

            let mut spend_days = 1;
            let mut curr_w = 0;
            for &w in weights.iter() {
                if curr_w + w > mid {
                    curr_w = w;
                    spend_days += 1;
                } else {
                    curr_w += w;
                }
            }

            if spend_days > days {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left.max(max)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5),
            15
        );
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::ship_within_days(vec![1, 2, 3, 1, 1], 4), 3);
    }
}
