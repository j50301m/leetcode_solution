/*
 * @lc app=leetcode id=2 lang=rust
 *
 * [2] Add Two Numbers
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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut curr1 = &l1;
        let mut curr2 = &l2;
        let mut over = 0;
        let mut stack = Vec::new();
        while curr1.is_some() || curr2.is_some() || over >0 {
            let val1 = match curr1.as_ref() {
                Some(node) => {
                    curr1 = &node.next;
                    node.val
                }
                None => 0,
            };
            let val2 = match curr2.as_ref() {
                Some(node) => {
                    curr2 = &node.next;
                    node.val
                }
                None => 0,
            };
            let sum = val1 + val2 + over;
            over = sum / 10;
            stack.push(sum % 10);
        }
    
        let mut next = None;
        for _ in 0..stack.len() {
            let node = Some(Box::new(ListNode {
                val: stack.pop().unwrap(),
                next: next,
            }));
            next = node;
        }
    
        next
    }
}
// @lc code=end

