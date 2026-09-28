/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */
#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_end: bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
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

    fn search(&self, word: String) -> bool {
        let bytes = word.as_bytes();
        let mut node = self;
        for i in 0..bytes.len() {
            let idx = (bytes[i] - b'a') as usize;
            if node.children[idx].is_none() {
                return false;
            }
            node = node.children[idx].as_ref().unwrap();
        }
        node.is_end
    }

    fn starts_with(&self, prefix: String) -> bool {
        let bytes = prefix.as_bytes();
        let mut node = self;
        for i in 0..bytes.len() {
            let idx = (bytes[i] - b'a') as usize;
            if node.children[idx].is_none() {
                return false;
            }
            node = node.children[idx].as_ref().unwrap();
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        // LeetCode example
        let mut trie = Trie::new();
        trie.insert("apple".to_string());
        assert!(trie.search("apple".to_string()));
        assert!(!trie.search("app".to_string()));
        assert!(trie.starts_with("app".to_string()));
        trie.insert("app".to_string());
        assert!(trie.search("app".to_string()));
    }

    #[test]
    fn case2() {
        // insert the longer word after the shorter one: "app" must stay a word
        let mut trie = Trie::new();
        trie.insert("app".to_string());
        trie.insert("apple".to_string());
        assert!(trie.search("app".to_string()));
        assert!(trie.search("apple".to_string()));
        assert!(!trie.search("appl".to_string()));
    }

    #[test]
    fn case3() {
        // missing branch, and a prefix longer than any word
        let mut trie = Trie::new();
        trie.insert("bat".to_string());
        assert!(!trie.search("cat".to_string()));
        assert!(!trie.starts_with("c".to_string()));
        assert!(!trie.starts_with("bath".to_string()));
        assert!(trie.starts_with("bat".to_string()));
    }
}
