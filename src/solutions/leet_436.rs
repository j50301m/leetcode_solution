struct Solution {}

impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let mut inter: Vec<_> = intervals
            .iter()
            .enumerate()
            .map(|(i, v)| (v[0], v[1], i as i32))
            .collect();
        let n = inter.len();
        let mut result = Vec::new();

        inter.sort_by_key(|(l, _r, _idx)| *l);
        for val in intervals.iter() {
            let pos = inter.partition_point(|&(x, _y, _i)| x < val[1]);
            if pos == n {
                result.push(-1);
            } else {
                result.push(inter[pos].2);
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::find_right_interval(vec![vec![3, 4], vec![2, 3], vec![1, 2]]),
            vec![-1, 0, 1]
        );
    }
}
