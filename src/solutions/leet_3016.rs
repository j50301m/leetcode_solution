use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut map = HashMap::new();
        let chars = word.chars();

        for char in chars {
            *map.entry(char).or_insert(0) += 1;
        }
        let mut list: Vec<(&char, &i32)> = map.iter().collect();
        list.sort_by(|a, b| b.1.cmp(a.1));

        let mut sum = 0;
        for (i, &(_, val)) in list.iter().enumerate() {
            sum += val * ((i / 8) as i32 + 1);
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::minimum_pushes("abcde".to_string()), 5);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::minimum_pushes("xyzxyzxyzxyz".to_string()), 12);
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::minimum_pushes("aabbccddeeffgghhiiiiii".to_string()),
            24
        );
    }
}
