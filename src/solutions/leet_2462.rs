use std::{cmp::Reverse, collections::BinaryHeap};

struct Solution {}

// impl Solution {
//     pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
//         let candidates = candidates as usize;
//         let k = k as usize;
//         let mut left = 0;
//         let mut right = costs.len() - 1;
//         let mut l_heap = BinaryHeap::new();
//         let mut r_heap = BinaryHeap::new();

//         for _ in 0..candidates {
//             if left > right {
//                 break;
//             }
//             l_heap.push(Reverse(costs[left]));
//             left += 1;
//             if left > right {
//                 break;
//             }
//             r_heap.push(Reverse(costs[right]));
//             right -= 1;
//         }

//         let mut sum = 0i64;
//         for _k in 0..k {
//             let l_cost: Option<&Reverse<i32>> = l_heap.peek();
//             let r_cost = r_heap.peek();
//             match (l_cost, r_cost) {
//                 (Some(&Reverse(l)), Some(&Reverse(r))) => {
//                     if l <= r {
//                         sum += l as i64;
//                         l_heap.pop();
//                         if left <= right {
//                             l_heap.push(Reverse(costs[left]));
//                             left += 1;
//                         }
//                     } else {
//                         sum += r as i64;
//                         r_heap.pop();
//                         if right >= left {
//                             r_heap.push(Reverse(costs[right]));
//                             right -= 1;
//                         }
//                     }
//                 }
//                 (Some(&Reverse(l)), None) => {
//                     sum += l as i64;
//                     l_heap.pop();
//                     if left <= right {
//                         l_heap.push(Reverse(costs[left]));
//                         left += 1;
//                     }
//                 }
//                 (None, Some(&Reverse(r))) => {
//                     sum += r as i64;
//                     r_heap.pop();
//                     if right >= left {
//                         r_heap.push(Reverse(costs[right]));
//                         right -= 1;
//                     }
//                 }
//                 _ => {
//                     unreachable!("error")
//                 }
//             }
//         }

//         sum
//     }
// }

impl Solution {
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        let (k, cand) = (k as usize, candidates as usize);
        let (mut left, mut right) = (0, costs.len() - 1);
        let (mut lo, mut hi) = (BinaryHeap::new(), BinaryHeap::new());
        let mut sum = 0i64;

        for _ in 0..k {
            while lo.len() < cand && left <= right {
                lo.push(Reverse(costs[left]));
                left += 1;
            }
            while hi.len() < cand && left <= right {
                hi.push(Reverse(costs[right]));
                right -= 1;
            }
            let taken = if lo.peek() >= hi.peek() {
                lo.pop()
            } else {
                hi.pop()
            };
            sum += taken.expect("k <= costs.len()").0 as i64;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::total_cost(vec![17, 12, 10, 2, 7, 2, 11, 20, 8], 3, 4),
            11
        );
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::total_cost(vec![1, 2, 4, 1], 3, 3), 4);
    }

    #[test]
    fn single() {
        assert_eq!(Solution::total_cost(vec![5], 1, 1), 5);
    }

    #[test]
    fn candidates_cover_whole_array() {
        // candidates 兩側加起來 >= len，所有人一開始就是候選人
        assert_eq!(Solution::total_cost(vec![3, 1, 2], 3, 2), 6);
    }

    #[test]
    fn hire_everyone() {
        // k == len，必須把每個元素都取到（含正中間那個）
        assert_eq!(Solution::total_cost(vec![10, 1, 9, 2, 8], 5, 1), 30);
    }

    #[test]
    fn middle_element_reachable() {
        // 中間的 1 只有在指標正確推進時才會進 heap
        assert_eq!(Solution::total_cost(vec![9, 9, 1, 9, 9], 3, 2), 19);
    }

    #[test]
    fn overlapping_candidates_no_double_count() {
        // len=4, candidates=3：兩側候選區間重疊，中間元素不可以被收兩次
        assert_eq!(Solution::total_cost(vec![1, 9, 2, 9], 4, 3), 21);
    }
}
