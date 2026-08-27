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

struct Solution {}

impl Solution {
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut odds = Box::new(ListNode::new(0));
        let mut even = Box::new(ListNode::new(0));
        let mut curr = &head;
        let mut index = 1;
        let mut odd_tail = &mut odds;
        let mut even_tail = &mut even;
        while let Some(node) = curr {
            let new_node = Box::new(ListNode::new(node.val));
            if index % 2 != 0 {
                odd_tail.next = Some(new_node);
                odd_tail = odd_tail.next.as_mut().unwrap();
            } else {
                even_tail.next = Some(new_node);
                even_tail = even_tail.next.as_mut().unwrap();
            }
            curr = &node.next;
            index += 1;
        }

        odd_tail.next = even.next;
        odds.next
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

    // 官方 example 1，奇數長度
    #[test]
    fn case_1() {
        assert_eq!(
            Solution::odd_even_list(build(&[1, 2, 3, 4, 5])),
            build(&[1, 3, 5, 2, 4])
        );
    }

    // 官方 example 2，偶數長度
    #[test]
    fn case_2() {
        assert_eq!(
            Solution::odd_even_list(build(&[2, 1, 3, 5, 6, 4, 7])),
            build(&[2, 3, 6, 7, 1, 5, 4])
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::odd_even_list(build(&[1, 2])), build(&[1, 2]));
    }

    #[test]
    fn case_4() {
        assert_eq!(Solution::odd_even_list(build(&[1])), build(&[1]));
    }

    #[test]
    fn case_5() {
        assert_eq!(Solution::odd_even_list(None), None);
    }
}
