/*
 * @lc app=leetcode id=19 lang=rust
 *
 * [19] Remove Nth Node From End of List
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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy =Some(Box::new(ListNode{
            val: 0,
            next :head
        }));
    
        // 先移動快指針到最後
        let mut len = 0;
        let mut curr = dummy.as_ref();
        while curr.unwrap().next.is_some() {
            len += 1;
            curr = curr.unwrap().next.as_ref();
        }
    
        // 找出要刪除的元素pos
        let pos = len - n;
        
        // 移動到 腰刪除的前一個元素
        let mut curr = &mut dummy;
        for _ in 0..pos {
            curr  = &mut  curr.as_mut().unwrap().next;
        }
    
        // 獲取需要的元素 重新接上
        let next = curr.as_mut().unwrap().next
        .as_mut().unwrap().next.take();
        curr.as_mut().unwrap().next = next;
    
        dummy.unwrap().next
    }
}
// @lc code=end

