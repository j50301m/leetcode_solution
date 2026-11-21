struct Solution {}

impl Solution {
    pub fn find_subtree_sizes(parent: Vec<i32>, s: String) -> Vec<i32> {
        let n = parent.len();
        let bytes = s.as_bytes();

        let mut parent_map = vec![Vec::new(); n];
        for child in 1..n {
            parent_map[parent[child] as usize].push(child);
        }

        let mut new_parent = vec![Vec::new(); n];
        let mut last_appeared = vec![-1; 26];
        Self::dfs(
            0,
            &parent,
            &parent_map,
            bytes,
            &mut last_appeared,
            &mut new_parent,
        );

        let mut ans = vec![0; n];
        Self::count_child(0, &new_parent, &mut ans);

        ans
    }

    fn count_child(curr: usize, new_parent: &[Vec<usize>], ans: &mut Vec<i32>) -> i32 {
        let mut count = 0;
        for &child in new_parent[curr].iter() {
            count += Self::count_child(child, new_parent, ans);
        }

        ans[curr] = count + 1;
        count + 1
    }

    fn dfs(
        curr: usize,
        parent: &[i32],
        parent_map: &[Vec<usize>],
        s: &[u8],
        last_appeared: &mut Vec<i32>,
        new_parent: &mut Vec<Vec<usize>>,
    ) {
        let last_pos = last_appeared[(s[curr] - b'a') as usize];
        let chosen_parent = if last_pos == -1 {
            parent[curr]
        } else {
            last_pos
        };

        if chosen_parent != -1 {
            new_parent[chosen_parent as usize].push(curr);
        }

        for &child in parent_map[curr].iter() {
            last_appeared[(s[curr] - b'a') as usize] = curr as i32;
            Self::dfs(child, parent, parent_map, s, last_appeared, new_parent);
            last_appeared[(s[curr] - b'a') as usize] = last_pos;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::find_subtree_sizes(vec![-1, 0, 0, 1, 1, 1], "abaabc".to_string()),
            vec![6, 3, 1, 1, 1, 1]
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::find_subtree_sizes(
                vec![-1, 10, 0, 12, 10, 18, 11, 12, 2, 3, 2, 2, 2, 0, 4, 11, 4, 2, 0],
                "babadabbdabcbaceeda".to_string()
            ),
            vec![19, 1, 15, 2, 3, 1, 1, 1, 1, 1, 5, 2, 4, 1, 1, 1, 1, 1, 2]
        );
    }
}
