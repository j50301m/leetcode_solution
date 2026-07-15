use std::collections::BinaryHeap;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        // Rust heap is max heap
        let mut heap = BinaryHeap::new();

        // Push all element in heap
        for list in lists {
            let mut cur = list;
            while let Some(node) = cur {
                heap.push(node.val);
                cur = node.next;
            }
        }

        // Pop element
        let mut head = None;
        while let Some(val) = heap.pop() {
            head = Some(Box::new(ListNode { val, next: head }));
        }

        return head;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn build(vals: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;
        for &v in vals.iter().rev() {
            head = Some(Box::new(ListNode { val: v, next: head }));
        }
        head
    }

    #[test]
    fn case_1() {
        let lists = vec![build(&[1, 4, 5]), build(&[1, 3, 4]), build(&[2, 6])];
        let result = Solution::merge_k_lists(lists);
        assert_eq!(result, build(&[1, 1, 2, 3, 4, 4, 5, 6]));
    }

    #[test]
    fn case_2() {
        let result = Solution::merge_k_lists(vec![]);
        assert_eq!(result, None);
    }
}
