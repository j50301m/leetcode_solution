struct Solution {}

impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, mut arr2: Vec<i32>, d: i32) -> i32 {
        let n = arr1.len();
        arr2.sort();

        let mut count = 0;
        for i in 0..n {
            let num = arr1[i];
            let up_side = num + d;
            let down_side = num - d;
            let down_pos = arr2.partition_point(|&x| x < down_side);
            let up_pos = arr2.partition_point(|&x| x <= up_side);
            if up_pos - down_pos == 0 {
                count += 1;
            };
        }
        count as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::find_the_distance_value(vec![4, 5, 8], vec![10, 9, 1, 8], 2),
            2
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::find_the_distance_value(vec![1, 4, 2, 3], vec![-4, -3, 6, 10, 20, 30], 3),
            2
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            Solution::find_the_distance_value(
                vec![2, 1, 100, 3],
                vec![-5, -23, -6, -1, 10, -6, 6],
                6
            ),
            1
        );
    }

    #[test]
    fn distance_exactly_d_is_too_close() {
        // |5 - 7| = 2 = d，算太近，所以 5 不算
        assert_eq!(Solution::find_the_distance_value(vec![5], vec![7], 2), 0);
    }

    #[test]
    fn zero_distance_only_rejects_equal_numbers() {
        // d = 0：只有 2 跟 arr2 的 2 一樣，1 和 3 都算
        assert_eq!(
            Solution::find_the_distance_value(vec![1, 2, 3], vec![2], 0),
            2
        );
    }
}
