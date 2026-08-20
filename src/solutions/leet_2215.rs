use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let mut set1: HashSet<i32> = HashSet::new();
        for &num in nums1.iter() {
            set1.insert(num);
        }
        let mut set2: HashSet<i32> = HashSet::new();
        for &num in nums2.iter() {
            set2.insert(num);
        }

        let list1 = set1.iter().copied().filter(|x| !set2.contains(x)).collect();
        let list2 = set2.iter().copied().filter(|x| !set1.contains(x)).collect();

        vec![list1, list2]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // HashSet 的迭代順序不保證，比對前一定要排序
    fn call(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let mut out = Solution::find_difference(nums1, nums2);
        for list in out.iter_mut() {
            list.sort();
        }
        out
    }

    #[test]
    fn case_1() {
        assert_eq!(call(vec![1, 2, 3], vec![2, 4, 6]), vec![vec![1, 3], vec![4, 6]]);
    }

    #[test]
    fn case_2() {
        assert_eq!(
            call(vec![1, 2, 3, 3], vec![1, 1, 2, 2]),
            vec![vec![3], vec![]]
        );
    }

    #[test]
    fn duplicates_are_deduped() {
        assert_eq!(call(vec![1, 1, 1], vec![2, 2]), vec![vec![1], vec![2]]);
    }

    #[test]
    fn identical_sets_give_two_empties() {
        assert_eq!(
            call(vec![3, 1, 2], vec![2, 3, 1]),
            vec![Vec::<i32>::new(), vec![]]
        );
    }

    #[test]
    fn fully_disjoint() {
        assert_eq!(call(vec![-1, -2], vec![5]), vec![vec![-2, -1], vec![5]]);
    }

    #[test]
    fn one_side_empty() {
        assert_eq!(call(vec![], vec![7, 7]), vec![vec![], vec![7]]);
    }
}
