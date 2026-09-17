struct Solution {}

impl Solution {
    pub fn find_buildings(heights: Vec<i32>) -> Vec<i32> {
        let n = heights.len();
        let mut max = 0;
        let mut results = Vec::new();
        for i in (0..n).rev() {
            if heights[i] > max {
                results.push(i as i32);
                max = heights[i];
            }
        }
        results.into_iter().rev().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::find_buildings(vec![4, 2, 3, 1]), vec![0, 2, 3]);
    }
}
