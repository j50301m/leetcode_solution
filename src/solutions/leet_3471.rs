use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn largest_integer(mut nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as usize;
        if k == 1 {
            let mut map: HashMap<i32, i32> = HashMap::new();
            for num in nums.iter() {
                *map.entry(*num).or_insert(0) += 1;
            }
            return map
                .iter()
                .filter(|(_, &count)| count == 1)
                .map(|(&x, _)| x)
                .max()
                .unwrap_or(-1);
        }
        if k == n {
            nums.sort();
            return nums[n - 1];
        }

        if nums[0] == nums[n - 1] {
            return -1;
        }

        let mut candidate1 = true;
        let mut candidate2 = true;
        for i in 1..n - 1 {
            if !candidate1 && !candidate2 {
                break;
            }
            if nums[i] == nums[0] {
                candidate1 = false;
            }
            if nums[i] == nums[n - 1] {
                candidate2 = false;
            }
        }

        if candidate1 && candidate2 {
            return nums[0].max(nums[n - 1]);
        } else if !candidate1 && candidate2 {
            return nums[n - 1];
        } else if candidate1 && !candidate2 {
            return nums[0];
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::largest_integer(vec![3, 9, 2, 1, 7], 1), 9);
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::largest_integer(vec![4, 4, 2, 2, 2, 0, 5, 3, 4, 4], 3),
            -1
        )
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::largest_integer(vec![10, 12, 9, 7, 10], 3), -1)
    }
}
