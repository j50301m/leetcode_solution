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
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut vec: Vec<i32> = Vec::new();
        let mut curr = head;
        while let Some(mut node) = curr {
            vec.push(node.val);
            curr = node.next.take();
        }

        let mut left = 0;
        let mut right = vec.len() - 1;
        let mut max_sum = 0;
        while left < right {
            max_sum = (vec[left] + vec[right]).max(max_sum);
            left += 1;
            right -= 1;
        }

        max_sum
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

    // 官方 example 1：兩對的和都是 6
    #[test]
    fn case1() {
        assert_eq!(Solution::pair_sum(build(&[5, 4, 2, 1])), 6);
    }

    // 官方 example 2：最大在外圈那一對
    #[test]
    fn case2() {
        assert_eq!(Solution::pair_sum(build(&[4, 2, 2, 3])), 7);
    }

    // 官方 example 3：最短長度 2，迴圈只跑一輪
    #[test]
    fn case3() {
        assert_eq!(Solution::pair_sum(build(&[1, 100000])), 100001);
    }

    // 最大值在第一對 -> 抓 max_sum 被後面的值蓋掉（忘了取 max）
    #[test]
    fn case4() {
        assert_eq!(Solution::pair_sum(build(&[100, 1, 2, 3, 4, 100])), 200);
    }

    // 最大值在最中間那一對，也就是迴圈最後一輪
    // -> 抓 while left < right 少跑一輪（例如寫成 left + 1 < right）
    #[test]
    fn case5() {
        assert_eq!(Solution::pair_sum(build(&[1, 2, 3, 50, 50, 3, 2, 1])), 100);
    }

    // 每一對的和都相等 -> 抓 twin 配對配錯
    // 若誤配成相鄰兩個，這筆會變成 11
    #[test]
    fn case6() {
        assert_eq!(Solution::pair_sum(build(&[1, 2, 3, 4, 5, 6])), 7);
    }

    // 全部同值，left/right 對撞的邊界
    #[test]
    fn case7() {
        assert_eq!(Solution::pair_sum(build(&[7, 7, 7, 7])), 14);
    }
}
