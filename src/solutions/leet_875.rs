struct Solution {}

impl Solution {
    pub fn min_eating_speed(mut piles: Vec<i32>, h: i32) -> i32 {
        piles.sort_unstable();
        let (mut lo, mut hi) = (1, piles[piles.len() - 1]);

        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let mut spent_hours = 0;
            for &pile in piles.iter() {
                spent_hours += pile / mid;
                if pile % mid != 0 {
                    spent_hours += 1;
                }
            }
            if spent_hours > h {
                lo = mid + 1;
            } else if spent_hours <= h {
                hi = mid;
            }
        }

        lo as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11], 8), 4);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 6), 23);
    }

    #[test]
    fn single_pile() {
        assert_eq!(Solution::min_eating_speed(vec![1000000000], 2), 500000000);
    }

    #[test]
    fn h_equals_len() {
        assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11], 4), 11);
    }

    // 時間很充裕，最佳速度吃完後還有剩下的小時數（spent_hours < h）
    #[test]
    fn slack_hours() {
        assert_eq!(Solution::min_eating_speed(vec![1, 2, 3, 4], 7), 2);
    }
}
