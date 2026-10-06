use std::collections::HashMap;

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */
struct TimeMap {
    map: HashMap<String, Vec<(i32, String)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        let entry = self.map.entry(key).or_insert(Vec::new());
        let pos = entry.partition_point(|(t, _)| *t <= timestamp);
        entry.insert(pos, (timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        let Some(val) = self.map.get(&key) else {
            return String::new();
        };

        let pos = val.partition_point(|(i, _)| *i <= timestamp);
        if pos == 0 {
            return String::new();
        }
        val[pos - 1].1.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let mut tm = TimeMap::new();
        tm.set("foo".to_string(), "bar".to_string(), 1);
        assert_eq!(tm.get("foo".to_string(), 1), "bar");
        assert_eq!(tm.get("foo".to_string(), 3), "bar");
        tm.set("foo".to_string(), "bar2".to_string(), 4);
        assert_eq!(tm.get("foo".to_string(), 4), "bar2");
        assert_eq!(tm.get("foo".to_string(), 5), "bar2");
    }

    #[test]
    fn between_timestamps() {
        let mut tm = TimeMap::new();
        tm.set("k".to_string(), "a".to_string(), 10);
        tm.set("k".to_string(), "b".to_string(), 20);
        assert_eq!(tm.get("k".to_string(), 15), "a");
    }

    #[test]
    fn before_first_timestamp() {
        let mut tm = TimeMap::new();
        tm.set("k".to_string(), "a".to_string(), 10);
        assert_eq!(tm.get("k".to_string(), 5), "");
    }

    #[test]
    fn missing_key() {
        let tm = TimeMap::new();
        assert_eq!(tm.get("nope".to_string(), 1), "");
    }
}
