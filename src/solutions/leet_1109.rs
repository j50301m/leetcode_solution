struct Solution {}

impl Solution {
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut diff = vec![0; n + 1];
        let mut ans = vec![0; n];
        for booking in bookings.iter() {
            let (start, end, reserved) = (booking[0] as usize, booking[1] as usize, booking[2]);
            diff[start - 1] += reserved;
            diff[end] -= reserved;
        }

        let mut sum = 0;
        for i in 0..diff.len() - 1 {
            sum += diff[i];
            ans[i] = sum;
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
            Solution::corp_flight_bookings(vec![vec![1, 2, 10], vec![2, 3, 20], vec![2, 5, 25]], 5),
            vec![10, 55, 45, 25, 25]
        );
    }
}
