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
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut vals = Vec::new();
        let mut curr_node = &head;
        while let Some(node) = curr_node {
            vals.push(node.val);
            curr_node = &node.next;
        }

        let mut indices = Vec::new();
        for i in 1..vals.len() - 1 {
            if vals[i] > vals[i - 1] && vals[i] > vals[i + 1] {
                indices.push(i);
            } else if vals[i] < vals[i - 1] && vals[i] < vals[i + 1] {
                indices.push(i);
            }
        }

        if indices.len() < 2 {
            return vec![-1, -1];
        }

        let mut min_distance = usize::MAX;
        for i in 0..indices.len() - 1 {
            min_distance = (indices[i + 1] - indices[i]).min(min_distance);
        }
        let max_distance = indices[indices.len() - 1] - indices[0];

        vec![min_distance as i32, max_distance as i32]
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

    // 官方 example 1，只有兩顆，沒有 critical point
    #[test]
    fn case_1() {
        assert_eq!(
            Solution::nodes_between_critical_points(build(&[3, 1])),
            vec![-1, -1]
        );
    }

    // 官方 example 2，critical 在 idx 2,4,5
    #[test]
    fn case_2() {
        assert_eq!(
            Solution::nodes_between_critical_points(build(&[5, 3, 1, 2, 5, 1, 2])),
            vec![1, 3]
        );
    }

    // 官方 example 3，有相鄰重複值，逼出「用 > 不是 >=」
    #[test]
    fn case_3() {
        assert_eq!(
            Solution::nodes_between_critical_points(build(&[1, 3, 2, 2, 3, 2, 2, 2, 7])),
            vec![3, 3]
        );
    }

    // 只有一個 critical point -> 必須是 -1,-1
    #[test]
    fn case_4() {
        assert_eq!(
            Solution::nodes_between_critical_points(build(&[1, 5, 2])),
            vec![-1, -1]
        );
    }

    // 全部相等，一個 critical 都沒有
    #[test]
    fn case_5() {
        assert_eq!(
            Solution::nodes_between_critical_points(build(&[2, 2, 2, 2])),
            vec![-1, -1]
        );
    }

    // 遞增，頭尾不算 critical -> 抓 for 迴圈邊界寫錯 (0..len 或 1..len)
    #[test]
    fn case_6() {
        assert_eq!(
            Solution::nodes_between_critical_points(build(&[1, 2, 3, 4, 5])),
            vec![-1, -1]
        );
    }

    // 兩個相鄰 critical，min 和 max 都是 1
    #[test]
    fn case_7() {
        assert_eq!(
            Solution::nodes_between_critical_points(build(&[1, 3, 2, 4])),
            vec![1, 1]
        );
    }

    // critical 在 idx 1,5,6：最小間距在最後一對 -> 抓「min 只算第一對」
    #[test]
    fn case_8() {
        assert_eq!(
            Solution::nodes_between_critical_points(build(&[1, 9, 8, 7, 6, 1, 9, 8])),
            vec![1, 5]
        );
    }

    // critical 在 idx 1,2,6：最小間距在第一對 -> 抓「min 只算最後一對」或 min 用 last-first
    #[test]
    fn case_9() {
        assert_eq!(
            Solution::nodes_between_critical_points(build(&[1, 9, 1, 2, 3, 4, 9, 1])),
            vec![1, 5]
        );
    }
}
