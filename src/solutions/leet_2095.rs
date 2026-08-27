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

struct Solution;

impl Solution {
    pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut length = 0;
        let mut curr = &head;
        while let Some(node) = curr {
            length += 1;
            curr = &node.next;
        }

        if length <= 1 {
            return None;
        }

        let mut prev = head.as_mut().unwrap();
        for _ in 0..length / 2 - 1 {
            prev = prev.next.as_mut().unwrap();
        }
        let mid = prev.next.take().unwrap();
        prev.next = mid.next;

        head
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

    // 官方 example 1，奇數
    #[test]
    fn case_1() {
        assert_eq!(
            Solution::delete_middle(build(&[1, 3, 4, 7, 1, 2, 6])),
            build(&[1, 3, 4, 1, 2, 6])
        );
    }

    // 官方 example 2，偶數取後面那顆
    #[test]
    fn case_2() {
        assert_eq!(
            Solution::delete_middle(build(&[1, 2, 3, 4])),
            build(&[1, 2, 4])
        );
    }

    // 官方 example 3
    #[test]
    fn case_3() {
        assert_eq!(Solution::delete_middle(build(&[2, 1])), build(&[2]));
    }

    // 唯一一顆 -> 刪完是空的，也是 length/2 - 1 underflow 的地雷
    #[test]
    fn case_4() {
        assert_eq!(Solution::delete_middle(build(&[1])), None);
    }

    #[test]
    fn case_5() {
        assert_eq!(Solution::delete_middle(build(&[1, 2, 3])), build(&[1, 3]));
    }
}
