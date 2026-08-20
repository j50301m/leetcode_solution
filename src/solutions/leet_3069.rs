struct Solution;

impl Solution {
    pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut vec1 = Vec::with_capacity(n);
        let mut vec2 = Vec::with_capacity(n);
        vec1.push(nums[0]);
        vec2.push(nums[1]);

        for i in 2..n {
            if vec1.last().unwrap() > vec2.last().unwrap() {
                vec1.push(nums[i]);
            } else {
                vec2.push(nums[i])
            }
        }

        vec1.extend(vec2);

        vec1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        // arr1=[2], arr2=[1]，2>1 → 3 進 arr1
        assert_eq!(Solution::result_array(vec![2, 1, 3]), vec![2, 3, 1]);
    }

    #[test]
    fn case_2() {
        // arr1=[5], arr2=[4]，5>4 → 3 進 arr1；之後 3<4 → 8 進 arr2
        assert_eq!(Solution::result_array(vec![5, 4, 3, 8]), vec![5, 3, 4, 8]);
    }

    #[test]
    fn min_length_goes_right() {
        // 1<3 → 2 進 arr2
        assert_eq!(Solution::result_array(vec![1, 3, 2]), vec![1, 3, 2]);
    }

    #[test]
    fn all_into_arr1() {
        // arr1 尾端永遠大於 1，全部進 arr1
        assert_eq!(
            Solution::result_array(vec![5, 1, 2, 3, 4]),
            vec![5, 2, 3, 4, 1]
        );
    }

    #[test]
    fn all_into_arr2() {
        // arr1 停在 1，之後每次比較都輸，全部進 arr2
        assert_eq!(
            Solution::result_array(vec![1, 2, 3, 4, 5]),
            vec![1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn negative_numbers() {
        // arr1=[-1], arr2=[-5]，-1>-5 → -3 進 arr1；-3>-5 → -4 進 arr1
        assert_eq!(
            Solution::result_array(vec![-1, -5, -3, -4]),
            vec![-1, -3, -4, -5]
        );
    }
}
