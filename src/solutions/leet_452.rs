struct Solution {}

impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_by_key(|x| x[1]);

        let mut cnt = 1;
        let mut right = points[0][1];
        for point in &points[1..] {
            if point[0] > right {
                right = point[1];
                cnt += 1;
            }
        }

        cnt
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::find_min_arrow_shots(vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]]),
            2
        );
    }
}
