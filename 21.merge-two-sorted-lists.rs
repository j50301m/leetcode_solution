/*
 * @lc app=leetcode id=21 lang=rust
 *
 * [21] Merge Two Sorted Lists
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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut curr1 = list1.as_ref();
        let mut curr2 = list2.as_ref();
        let mut stack = Vec::new();
        while curr1.is_some() || curr2.is_some() {
            if curr1.is_none() {
                stack.push(curr2.unwrap().val);
                curr2 = curr2.unwrap().next.as_ref();
            }
            else if curr2.is_none() {
                stack.push(curr1.unwrap().val);
                curr1 = curr1.unwrap().next.as_ref();
            }
            else if curr1.unwrap().val < curr2.unwrap().val {
                stack.push(curr1.unwrap().val);
                curr1 = curr1.unwrap().next.as_ref();
            } else {
                stack.push(curr2.unwrap().val);
                curr2 = curr2.unwrap().next.as_ref();
            }
        }
    
        let mut next_node: Option<Box<ListNode>> = None;
        while let Some(val) = stack.pop() {
            let mut node = Box::new(ListNode::new(val));
            node.next = next_node.clone();
            next_node = Some(node);
        }
    
        next_node
    }
}
// @lc code=end

