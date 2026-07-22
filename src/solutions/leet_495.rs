struct Solution;

impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let mut poison_end = -1;
        let mut total = 0;
        for i in time_series {
            let new = i + duration - 1;
            if i > poison_end {
                total += duration;
            } else {
                total += new - poison_end;
            }
            poison_end = new;
        }

        total
    }
}

#[cfg(test)]
mod test {
    use std::assert_eq;

    use super::*;

    #[test]
    fn case_1() {
        let time_series = vec![1, 4];
        let duration = 2;
        let result = Solution::find_poisoned_duration(time_series, duration);
        assert_eq!(result, 4);
    }

    #[test]
    fn case_2() {
        let time_series = vec![0, 1, 2];
        let duration = 1;
        let result = Solution::find_poisoned_duration(time_series, duration);
        assert_eq!(result, 3);
    }

    #[test]
    fn case_3() {
        let time_series = vec![1, 2];
        let duration = 2;
        let result = Solution::find_poisoned_duration(time_series, duration);
        assert_eq!(result, 3);
    }
}
