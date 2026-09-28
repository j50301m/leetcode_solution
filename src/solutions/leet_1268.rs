use core::str;

struct Solution {}

impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut trie = Trie::new();
        for p in products {
            trie.insert(p);
        }

        trie.bfs(search_word.as_bytes(), 3)
    }
}

#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_end: bool,
}

impl Trie {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, word: String) {
        let bytes = word.as_bytes();
        let mut node = self;

        for i in 0..bytes.len() {
            let idx = (bytes[i] - b'a') as usize;
            if node.children[idx].is_none() {
                node.children[idx] = Some(Box::new(Trie::new()));
            }
            node = node.children[idx].as_mut().unwrap();
        }
        node.is_end = true;
    }

    fn bfs(&self, word: &[u8], k: usize) -> Vec<Vec<String>> {
        let mut node = self;
        let mut ans = Vec::new();
        let mut is_match = true;
        for i in 0..word.len() {
            let idx = (word[i] - b'a') as usize;
            let mut result = Vec::new();
            if !is_match || node.children[idx].is_none() {
                ans.push(result);
                is_match = false;
                continue;
            }
            node = node.children[idx].as_ref().unwrap();
            let mut prefix = str::from_utf8(&word[..=i].to_vec()).unwrap().to_string();
            Self::dfs(k, node, &mut prefix, &mut result);
            ans.push(result)
        }

        ans
    }

    fn dfs(k: usize, node: &Trie, prefix: &mut String, result: &mut Vec<String>) {
        if result.len() == k {
            return;
        }

        if node.is_end {
            result.push(prefix.clone());
        }

        for (i, child) in node
            .children
            .iter()
            .enumerate()
            .filter(|(_, x)| x.is_some())
            .map(|(i, x)| (i, x.as_ref().unwrap()))
        {
            prefix.push(((i as u8) + b'a') as char);
            Self::dfs(k, child, prefix, result);
            prefix.remove(prefix.len() - 1);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn to_vec(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn case1() {
        let products = to_vec(&["mobile", "mouse", "moneypot", "monitor", "mousepad"]);
        let expected = vec![
            to_vec(&["mobile", "moneypot", "monitor"]),
            to_vec(&["mobile", "moneypot", "monitor"]),
            to_vec(&["mouse", "mousepad"]),
            to_vec(&["mouse", "mousepad"]),
            to_vec(&["mouse", "mousepad"]),
        ];
        assert_eq!(
            Solution::suggested_products(products, "mouse".to_string()),
            expected
        );
    }

    #[test]
    fn case2() {
        let products = to_vec(&["havana"]);
        let expected = vec![to_vec(&["havana"]); 6];
        assert_eq!(
            Solution::suggested_products(products, "havana".to_string()),
            expected
        );
    }

    #[test]
    fn case3() {
        let products = to_vec(&["bags", "baggage", "banner", "box", "cloths"]);
        let expected = vec![
            to_vec(&["baggage", "bags", "banner"]),
            to_vec(&["baggage", "bags", "banner"]),
            to_vec(&["baggage", "bags"]),
            to_vec(&["bags"]),
        ];
        assert_eq!(
            Solution::suggested_products(products, "bags".to_string()),
            expected
        );
    }

    // 前綴走到一半就斷掉：之後每個字元都要回傳空的 Vec
    #[test]
    fn prefix_not_found() {
        let products = to_vec(&["havana"]);
        let expected: Vec<Vec<String>> = vec![vec![]; 7];
        assert_eq!(
            Solution::suggested_products(products, "tatiana".to_string()),
            expected
        );
    }
}

// 第一個字元就斷了，後面的字元剛好在 root 底下存在，也不能算有找到
#[cfg(test)]
mod test_broken_prefix {
    use super::*;

    #[test]
    fn broken_prefix() {
        let products = vec!["b".to_string()];
        let expected: Vec<Vec<String>> = vec![vec![]; 2];
        assert_eq!(
            Solution::suggested_products(products, "ab".to_string()),
            expected
        );
    }
}
