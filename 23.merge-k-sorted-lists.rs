/*
 * @lc app=leetcode id=23 lang=rust
 *
 * [23] Merge k Sorted Lists
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = std::collections::BinaryHeap::new();
        
        // Push in heap
        for mut list in lists.into_iter() {
            while let Some(node) = list.take() {
                heap.push(node.val);
                list = node.next;
            }
        }
    
        // Pop from heap
        let mut res = None;
        while let Some(val) = heap.pop() {
            let node = Box::new(ListNode{
                val:val,
                next:res,
            });
            res = Some(node);
        }
        
        res
    }

    // pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    //     if lists.len() == 0 {
    //         return None;
    //     }
    
    //     if lists.len() == 1 {
    //         return lists.remove(0)
    //     }
    
    //     let mut sorted = lists.remove(0);
    //     for element in lists.into_iter() {
    //         let result = Self::merge_2_lists(sorted.take(), element);
    //         sorted = result;
    //     }
    
    //     sorted
    // }
    
    // pub fn merge_2_lists(
    //     list1: Option<Box<ListNode>>,
    //     list2: Option<Box<ListNode>>,
    // ) -> Option<Box<ListNode>> {
    //     if list1.is_none() {
    //         return list2;
    //     }
    
    //     if list2.is_none() {
    //         return list1;
    //     }
    
    //     let mut temp = list1;
    //     let mut list2 = list2;
    
    //     if list2.as_ref()?.val < temp.as_ref()?.val {
    //         std::mem::swap(&mut temp, &mut list2);
    //     }
    
    //     let sorted = Self::merge_2_lists(temp.as_mut().unwrap().next.take(), list2);
    //     temp.as_mut().unwrap().next = sorted;
    
    //     temp
    // }
}
// @lc code=end

