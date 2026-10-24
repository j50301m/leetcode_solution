struct Solution {}

impl Solution {
    pub fn count_days_together(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        const MONTHS: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut prefix_days = vec![0; 13];
        for i in 0..MONTHS.len() {
            prefix_days[i + 1] = prefix_days[i] + MONTHS[i];
        }

        let a1 = Self::convert_to_day(arrive_alice, &prefix_days);
        let a2 = Self::convert_to_day(leave_alice, &prefix_days);
        let b1 = Self::convert_to_day(arrive_bob, &prefix_days);
        let b2 = Self::convert_to_day(leave_bob, &prefix_days);

        let arrive = a1.max(b1);
        let leave = a2.min(b2);

        (leave - arrive + 1).max(0)
    }

    fn convert_to_day(date: String, prefix_days: &Vec<i32>) -> i32 {
        let slice: Vec<&str> = date.split('-').collect();
        let month = slice[0].parse::<usize>().unwrap();
        let day = slice[1].parse::<i32>().unwrap();

        prefix_days[month - 1] + day
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::count_days_together(
                "08-15".to_string(),
                "08-18".to_string(),
                "08-16".to_string(),
                "08-19".to_string()
            ),
            3
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::count_days_together(
                "01-01".to_string(),
                "12-31".to_string(),
                "01-01".to_string(),
                "12-31".to_string()
            ),
            365
        );
    }
}
