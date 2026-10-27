use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn path_sum(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for &num in nums.iter() {
            let (layer, node, val) = Self::parse(num);
            map.insert((layer, node), val);
        }

        let (_, val) = Self::dfs(1, 1, &map);
        val
    }

    fn dfs(layer: i32, node: i32, map: &HashMap<(i32, i32), i32>) -> (i32, i32) {
        let Some(&val) = map.get(&(layer, node)) else {
            return (0, 0);
        };

        let (l_leafs, l_val) = Self::dfs(layer + 1, 2 * (node - 1) + 1, map);
        let (r_leafs, r_val) = Self::dfs(layer + 1, 2 * (node - 1) + 2, map);

        if l_leafs == 0 && r_leafs == 0 {
            return (1, val);
        }

        let total_leafs = l_leafs + r_leafs;
        (total_leafs, val * total_leafs + l_val + r_val)
    }

    fn parse(mut num: i32) -> (i32, i32, i32) {
        let val = num % 10;
        num /= 10;
        let node = num % 10;
        num /= 10;
        let layer = num % 10;

        (layer, node, val)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::path_sum(vec![113, 215, 221]), 12);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::path_sum(vec![113, 229, 349, 470, 485]), 47);
    }
}
