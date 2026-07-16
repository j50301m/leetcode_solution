use std::collections::HashMap;

struct Solution;

struct TwoSum {
    map: HashMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TwoSum {
    fn new() -> Self {
        TwoSum {
            map: HashMap::new(),
        }
    }

    fn add(&mut self, number: i32) {
        if let Some(&val) = self.map.get(&number) {
            self.map.insert(number, val + 1);
            return;
        }
        self.map.insert(number, 1);
    }

    fn find(&self, value: i32) -> bool {
        for (&num, &time) in self.map.iter() {
            let target = value - num;
            if target == num {
                if time > 1 {
                    return true;
                }
            } else if self.map.contains_key(&target) {
                return true;
            }
        }

        false
    }
}

// /**
//  * Your TwoSum object will be instantiated and called as such:
//  * let obj = TwoSum::new();
//  * obj.add(number);
//  * let ret_2: bool = obj.find(value);
//  */
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let mut ins = TwoSum::new();
        ins.add(1);
        ins.add(3);
        ins.add(5);
        assert_eq!(ins.find(4), true);
        assert_eq!(ins.find(7), false);
    }
}
