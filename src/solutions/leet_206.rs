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
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut curr = &mut head;
        let mut vec: Vec<i32> = Vec::new();
        while let Some(node) = curr {
            vec.push(node.val);
            curr = &mut node.next;
        }

        let mut dummy = Box::new(ListNode::new(0));
        let mut curr = &mut dummy;
        for &val in vec.iter().rev() {
            let node = Box::new(ListNode::new(val));
            curr.next = Some(node);
            curr = curr.next.as_mut().unwrap();
        }

        dummy.next
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

    // 官方 example 1
    #[test]
    fn case_1() {
        assert_eq!(
            Solution::reverse_list(build(&[1, 2, 3, 4, 5])),
            build(&[5, 4, 3, 2, 1])
        );
    }

    // 官方 example 2，兩顆
    #[test]
    fn case_2() {
        assert_eq!(Solution::reverse_list(build(&[1, 2])), build(&[2, 1]));
    }

    // 官方 example 3，空 list
    #[test]
    fn case_3() {
        assert_eq!(Solution::reverse_list(None), None);
    }

    // 一顆，反轉前後長一樣 -> 抓不到「忘記反轉」，但抓得到 dummy 洩漏出去
    #[test]
    fn case_4() {
        assert_eq!(Solution::reverse_list(build(&[7])), build(&[7]));
    }

    // 回文，只有值相同、順序不同的 case 才逼得出 rev()
    #[test]
    fn case_5() {
        assert_eq!(
            Solution::reverse_list(build(&[1, 2, 2, 3])),
            build(&[3, 2, 2, 1])
        );
    }
}
