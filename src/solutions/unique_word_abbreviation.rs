use std::collections::HashMap;

pub struct ValidWordAbbr {
    abbreviation: HashMap<String, Option<String>>, // Key: abbr, Val: Some(唯一擁有者) / None(多字共用)
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ValidWordAbbr {
    pub fn new(dictionary: Vec<String>) -> Self {
        // Create a abbr map
        let mut map = HashMap::new();
        for s in dictionary {
            // Convert to abbreviation
            let abbr = Self::to_abbr(&s);

            //  Build a abbr map
            match map.get(&abbr) {
                None => {
                    map.insert(abbr, Some(s)); // 第一次見到這個縮寫
                }
                Some(Some(w)) if *w != s => {
                    map.insert(abbr, None); // 不同的字撞到同縮寫
                }
                _ => {} // 同一個字重複出現，維持原狀
            }
        }

        ValidWordAbbr { abbreviation: map }
    }

    pub fn is_unique(&self, word: String) -> bool {
        let abbr = Self::to_abbr(&word);
        match self.abbreviation.get(&abbr) {
            None => true,                // 沒人用這縮寫
            Some(Some(w)) => *w == word, // 唯一擁有者就是自己才算 unique
            Some(None) => false,         // 多個不同的字共用
        }
    }

    fn to_abbr(s: &String) -> String {
        if s.len() <= 2 {
            return s.clone(); // 長度 <= 2 縮寫就是自己，也避免 len-2 underflow
        }
        let bytes = s.as_bytes();
        let first = bytes[0] as char;
        let last = bytes[bytes.len() - 1] as char;
        format!("{}{}{}", first, s.len() - 2, last)
    }
}

// /**
//  * Your ValidWordAbbr object will be instantiated and called as such:
//  * let obj = ValidWordAbbr::new(dictionary);
//  * let ret_1: bool = obj.is_unique(word);
//  */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_case() {
        let dict = ["deer", "door", "cake", "card"].map(String::from).to_vec();
        let obj = ValidWordAbbr::new(dict);
        assert!(!obj.is_unique("dear".to_string()));
        assert!(obj.is_unique("cart".to_string()));
        assert!(!obj.is_unique("cane".to_string()));
        assert!(obj.is_unique("make".to_string()));
        assert!(obj.is_unique("cake".to_string())); // 之前錯的案例
    }
}
